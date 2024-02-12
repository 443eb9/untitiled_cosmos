use rand::Rng;
use rand_distr::Uniform;

use crate::{
    assets::settings::PackedStarInfo,
    consts::{self, greek_alphabets},
    math::{self},
    sci::{
        chemistry::{MatterState, SubstanceContent},
        physics,
    },
    sim::{
        bundles::{MoonBundle, PlanetBundle, StarBundle},
        components::{
            CelestialBodyAtmosphere, CelestialBodyColor, CelestialBodyCrust,
            CelestialBodyEffectiveTemp, CelestialBodyId, CelestialBodyName,
            CelestialBodySubstanceProps, Moon, Planet, PlanetType, Star, StarLuminosity,
        },
        resources::CelestialBody,
    },
};

use super::{
    distr::{
        self, GasGiantCompositionDistribution, IceGiantCompositionDistribution,
        RockyAtmosphereCompositionDistribution, RockyBodyAtmoDensityDistribution,
        RockyBodyCrustDensityDistribution, RockyCrustCompositionDistribution,
        StarCompositionDistribution,
    },
    GalaxyGenerator,
};

impl<'a> GalaxyGenerator<'a> {
    pub fn gen_star_props(
        &mut self,
        id: CelestialBodyId,
        lerp_factor: f64,
        bound_floor: &PackedStarInfo,
        bound_ceil: &PackedStarInfo,
    ) -> StarBundle {
        let name_distr = Uniform::new(0, self.constellation_names.len());
        let constellation = loop {
            let i = self.rng.sample(name_distr);
            let name = self.constellation_names.get_cloned(i);
            if !self.existed_constellation_names.contains(&name) {
                break name;
            }
        };
        let prefix = greek_alphabets::ALPHABETS_GREEK[self
            .rng
            .gen_range(0..greek_alphabets::ALPHABETS_GREEK.len())]
        .to_string();
        let lerped = bound_floor.lerp(bound_ceil, lerp_factor);
        let content = self.rng.sample(StarCompositionDistribution);
        let density = math::mass_radius_to_density(lerped.mass, lerped.radius);

        StarBundle {
            id,
            name: CelestialBodyName(format!("{} {}", prefix, constellation)),
            class: lerped.class,
            composition: CelestialBodyCrust { content, density },
            effective_temp: CelestialBodyEffectiveTemp(lerped.effective_temp),
            luminosity: StarLuminosity(lerped.luminosity),
            color: CelestialBodyColor(lerped.color.into()),
            tag: Star,
        }
    }

    pub fn gen_planet_props(
        &mut self,
        id: CelestialBodyId,
        planet: &CelestialBody,
        density: f64,
        ty: PlanetType,
        star: &CelestialBody,
        star_bundle: &StarBundle,
    ) -> (
        PlanetBundle,
        Option<CelestialBodyCrust>,
        Option<CelestialBodyAtmosphere>,
    ) {
        let mut atmosphere = match ty {
            PlanetType::GasGiant => Some(CelestialBodyAtmosphere {
                content: self.rng.sample(GasGiantCompositionDistribution),
                density,
            }),
            PlanetType::IceGiant => Some(CelestialBodyAtmosphere {
                content: self.rng.sample(IceGiantCompositionDistribution),
                density,
            }),
            PlanetType::Rocky => {
                let density = self.rng.sample(RockyBodyAtmoDensityDistribution);
                if density < consts::ATMOSPHERE_DENSITY_MIN {
                    None
                } else {
                    Some(CelestialBodyAtmosphere {
                        content: self.rng.sample(RockyAtmosphereCompositionDistribution),
                        density,
                    })
                }
            }
        };

        let mut eff_temp = physics::planetary_eq_temp_from_temp(
            star_bundle.effective_temp.0,
            star.radius(),
            0.,
            (planet.pos() - star.pos()).dot(consts::DEFAULT_BODY_EXTEND_AXIS),
        ) * consts::PLANET_EFFCETIVE_TEMP_SCALE;

        let mut crust = match ty {
            PlanetType::GasGiant | PlanetType::IceGiant => None,
            PlanetType::Rocky => Some(CelestialBodyCrust {
                content: self.rng.sample(RockyCrustCompositionDistribution),
                density,
            }),
        };

        let substance_props = self.adjust(&mut atmosphere, &mut crust, &mut eff_temp);
        let color = match ty {
            PlanetType::GasGiant | PlanetType::IceGiant => {
                let mut color = atmosphere
                    .as_ref()
                    .unwrap()
                    .content
                    .estimate_color(self.substance_assets, MatterState::Gas);
                color.a = 1.;
                color
            }
            PlanetType::Rocky => {
                let mut color = crust
                    .as_ref()
                    .unwrap()
                    .content
                    .estimate_color(self.substance_assets, MatterState::Solid);

                if let Some(atmo) = &atmosphere {
                    let atmo_col = atmo
                        .content
                        .estimate_color(self.substance_assets, MatterState::Gas);
                    let opacity = distr::rocky_body_atmo_density_to_opacity(atmo.density);
                    color = color.lerp(atmo_col, opacity as f32);
                }

                color.a = 1.;
                color
            }
        };

        (
            PlanetBundle {
                id,
                name: CelestialBodyName("".to_string()),
                ty,
                color: CelestialBodyColor(color.into()),
                effective_temp: CelestialBodyEffectiveTemp(eff_temp),
                substance_props,
                tag: Planet,
            },
            crust,
            atmosphere,
        )
    }

    pub fn gen_moon_props(
        &mut self,
        id: CelestialBodyId,
        moon: &CelestialBody,
        star: &CelestialBody,
        star_bundle: &StarBundle,
    ) -> (
        MoonBundle,
        CelestialBodyCrust,
        Option<CelestialBodyAtmosphere>,
    ) {
        let mut atmosphere = {
            let density = self.rng.sample(RockyBodyAtmoDensityDistribution);
            if density < consts::ATMOSPHERE_DENSITY_MIN {
                None
            } else {
                Some(CelestialBodyAtmosphere {
                    content: self.rng.sample(RockyAtmosphereCompositionDistribution),
                    density,
                })
            }
        };

        let mut eff_temp = physics::planetary_eq_temp_from_temp(
            star_bundle.effective_temp.0,
            star.radius(),
            0.,
            (moon.pos() - star.pos()).dot(consts::DEFAULT_BODY_EXTEND_AXIS),
        ) * consts::PLANET_EFFCETIVE_TEMP_SCALE;

        let mut crust = Some(CelestialBodyCrust {
            content: self.rng.sample(RockyCrustCompositionDistribution),
            density: self.rng.sample(RockyBodyCrustDensityDistribution),
        });

        let substance_props = self.adjust(&mut atmosphere, &mut crust, &mut eff_temp);
        let crust = crust.unwrap();
        let mut color = crust
            .content
            .estimate_color(self.substance_assets, MatterState::Solid);

        if let Some(atmo) = &atmosphere {
            let atmo_col = atmo
                .content
                .estimate_color(self.substance_assets, MatterState::Gas);
            let opacity = distr::rocky_body_atmo_density_to_opacity(atmo.density);
            color = color.lerp(atmo_col, opacity as f32);
        }

        (
            MoonBundle {
                id,
                name: CelestialBodyName("".to_string()),
                color: CelestialBodyColor(color.into()),
                effective_temp: CelestialBodyEffectiveTemp(eff_temp),
                substance_props,
                tag: Moon,
            },
            crust,
            atmosphere,
        )
    }

    fn adjust(
        &self,
        atmo: &mut Option<CelestialBodyAtmosphere>,
        crust: &mut Option<CelestialBodyCrust>,
        eff_temp: &mut f64,
    ) -> CelestialBodySubstanceProps {
        if let Some(atmo) = atmo {
            let density = atmo.density * 1000.;
            let greenhouse_coeff =
                density
                    * atmo.content.clone().normalized().iter().fold(
                        0.,
                        |mut acc, (sub, content)| {
                            if sub.is_greenhouse_gas() {
                                acc += *content;
                            }
                            acc
                        },
                    );
            let coeff = 1. / (1. + (-greenhouse_coeff * density).exp()) + 0.5;
            *eff_temp *= coeff;
        }

        let substance_props = atmo
            .as_ref()
            .map(|atmo| self.substance_assets.generate_props_on(atmo.density))
            .unwrap_or(self.substance_assets.generate_in_vaccum());

        if let Some(crust) = crust {
            crust.content = SubstanceContent::new(
                crust
                    .content
                    .iter()
                    .filter_map(|(sub, content)| {
                        let prop = substance_props.get(*sub);
                        if prop.boiling_point < 0. {
                            return Some((*sub, *content));
                        }

                        if *eff_temp > prop.boiling_point {
                            if let Some(vapor) = prop.vapor {
                                if let Some(atmo) = atmo {
                                    atmo.content.get_mut().insert(vapor, *content);
                                }
                            }
                            None
                        } else {
                            Some((*sub, *content))
                        }
                    })
                    .collect(),
            );
        }

        if let Some(atmo) = atmo {
            atmo.content.normalize();
        }
        if let Some(crust) = crust {
            crust.content.normalize();
        }

        CelestialBodySubstanceProps::new(substance_props.to_vec())
    }
}
