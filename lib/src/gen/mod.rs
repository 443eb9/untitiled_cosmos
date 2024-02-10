use bevy::{
    asset::Assets,
    log::{error, info},
    math::DVec2,
    render::mesh::Mesh,
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rand_distr::Uniform;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    assets::{settings::StarProperties, MaterialAssets, MeshAssets},
    consts,
    math::{self, physics},
    sim::{
        bundles::{CelestialBodyBundle, MoonBundle, PlanetBundle, StarBundle},
        components::{CelestialBodyId, CelestialBodyName, Moon, Planet, PlanetType},
        resources::{CelestialBody, Galaxy},
    },
};

use self::{
    distr::{
        GiantPlanetDensityDistribution, MoonDensityDistribution, MoonMassDistribution,
        MoonSmiDistIncreCoeffDistribution, PlanetMassDistribution, PlanetSmiDistDistribution,
        RockyPlanetDensityDistribution, StarMassDistribution, StarPosDistribution,
    },
    err::{MoonGenerationError, PlanetGenerationError},
};

pub mod distr;
pub mod err;
pub mod props;

pub struct PlanetGenerationConfig {
    pub num_coeff: Uniform<f32>,
    /// Minimum linear speed of a planet at farthest point from the star.
    pub min_revl_spd: f64,
    pub sma_smi_ratio: Uniform<f64>,
}

pub struct MoonGenerationConfig {
    pub num_coeff: Uniform<f32>,
    pub sma_smi_ratio: Uniform<f64>,
}

pub struct GalaxyGeneratorConfig {
    pub seed: u64,
    pub galaxy_name: String,
    pub galaxy_radius: f64,
    pub num_stars: usize,
    pub pln_cfg: PlanetGenerationConfig,
    pub moon_cfg: MoonGenerationConfig,
}

impl GalaxyGeneratorConfig {
    pub fn new_debug() -> Self {
        Self {
            seed: 0,
            galaxy_name: "Milky Way".to_string(),
            galaxy_radius: 1e3,
            num_stars: 1,
            pln_cfg: PlanetGenerationConfig {
                num_coeff: Uniform::new(0.6, 1.1),
                min_revl_spd: 5.,
                sma_smi_ratio: Uniform::new(1., 1.2),
            },
            moon_cfg: MoonGenerationConfig {
                num_coeff: Uniform::new(0.9, 1.2),
                sma_smi_ratio: Uniform::new(1., 1.05),
            },
        }
    }
}

pub struct GalaxyGenerator<'a> {
    cfg: GalaxyGeneratorConfig,
    rng: StdRng,
    star_props: &'a StarProperties,
    mesh_asstes: &'a mut MeshAssets,
    material_assets: &'a mut MaterialAssets,
    meshes: &'a mut Assets<Mesh>,
    materials: &'a mut Assets<ColorMaterial>,
    galaxy: Galaxy,
    bundles: Vec<(CelestialBodyBundle, MaterialMesh2dBundle<ColorMaterial>)>,
    smi_dist: Vec<f64>,
    sma_dist: Vec<f64>,
}

impl<'a> GalaxyGenerator<'a> {
    pub fn new(
        config: GalaxyGeneratorConfig,
        star_props: &'a StarProperties,
        mesh_asstes: &'a mut MeshAssets,
        material_assets: &'a mut MaterialAssets,
        meshes: &'a mut Assets<Mesh>,
        materials: &'a mut Assets<ColorMaterial>,
    ) -> Self {
        Self {
            rng: SeedableRng::seed_from_u64(config.seed),
            cfg: config,
            star_props,
            mesh_asstes,
            material_assets,
            meshes,
            materials,
            galaxy: Galaxy::default(),
            bundles: Vec::new(),
            smi_dist: Vec::new(),
            sma_dist: Vec::new(),
        }
    }

    pub fn transfer_result(
        &mut self,
        galaxy: &mut Galaxy,
        bundles: &mut Vec<(CelestialBodyBundle, MaterialMesh2dBundle<ColorMaterial>)>,
    ) {
        std::mem::swap(&mut self.galaxy, galaxy);
        std::mem::swap(&mut self.bundles, bundles);
    }

    pub fn generate(&mut self) {
        let start = std::time::SystemTime::now();
        info!("Galaxy generation started");

        for system_id in 0..self.cfg.num_stars {
            info!(
                "Generating stars {}/{}[{}]",
                system_id, self.cfg.num_stars, self.cfg.seed
            );

            let (star, star_bundle) = self.gen_star();
            let num_planets = distr::max_num_planets(
                star_bundle.class.to_index(),
                self.cfg.pln_cfg.num_coeff,
                &mut self.rng,
            );
            let system_edge =
                physics::linear_spd_to_dist(self.cfg.pln_cfg.min_revl_spd, star.mass());

            for planet_systemic_id in 0..num_planets {
                info!(
                    "Generating planets {}/{}[{}, {}]",
                    planet_systemic_id, num_planets, system_id, self.cfg.seed
                );
                let (planet, planet_id) = match self.gen_planet(&star, system_id, system_edge) {
                    Ok(body) => body,
                    Err(err) => {
                        error!("Planet generation failed: {}", err);
                        continue;
                    }
                };

                let num_moons =
                    distr::max_num_moons(planet.mass(), self.cfg.moon_cfg.num_coeff, &mut self.rng);

                for moon_systemic_id in 0..num_moons {
                    info!(
                        "Generating moons {}/{}[{}, {}, {}]",
                        moon_systemic_id, num_moons, system_id, planet_systemic_id, self.cfg.seed
                    );
                    match self.gen_moon(system_id, &star, planet_id, &planet) {
                        Ok(_) => {}
                        Err(err) => error!("Moon generation failed: {}", err),
                    }
                }
            }
        }
        let elapsed = start.elapsed().unwrap();

        info!("Galaxy generation finished in {}ms", elapsed.as_millis());
    }

    fn gen_star(&mut self) -> (CelestialBody, StarBundle) {
        let spd = StarPosDistribution::new(self.cfg.galaxy_radius);
        let smd = StarMassDistribution;

        let mass = self.rng.sample(smd);
        let pos = {
            loop {
                let pos = self.rng.sample(spd);
                let num_confl = self
                    .galaxy
                    .systems()
                    .par_iter()
                    .filter(|system| {
                        let other = self.galaxy.get_body(system.bodies[0]).unwrap();
                        let f = physics::force_between(
                            &CelestialBody::new(pos, 0., mass, DVec2::ZERO),
                            other,
                        );
                        let a1 = f / mass;
                        let a2 = f / other.mass();

                        a1 > consts::STAR_ACC_THRESHOLD || a2 > consts::STAR_ACC_THRESHOLD
                    })
                    .collect::<Vec<_>>()
                    .len();
                if num_confl == 0 {
                    break pos;
                }
            }
        };

        let (bound_floor, bound_ceil) = self.star_props.find_bound(mass, |info| info.mass);
        let percent = (mass - bound_floor.mass) / (bound_ceil.mass - bound_floor.mass);
        let radius = bound_floor.radius + (bound_ceil.radius - bound_floor.radius) * percent;

        let star = CelestialBody::new(
            pos,
            radius * consts::SUN_RADIUS * consts::STAR_RADIUS_SCALE,
            mass * consts::SUN_MASS * consts::STAR_MASS_SCALE,
            DVec2::ZERO,
        );

        self.galaxy.create_system();

        let (id, systemic_id) = self.galaxy.add_body(self.galaxy.system_count() - 1, star);

        let mesh = Mesh2dHandle(self.mesh_asstes.generate(self.meshes, id, star.radius()));
        let (material, color) = self
            .material_assets
            .generate(self.materials, id, &mut self.rng);

        self.galaxy.set_color(id, color);

        let bundle = StarBundle {
            id,
            systemic_id,
            name: CelestialBodyName("".to_string()),
            class: bound_floor.class,
        };

        self.bundles.push((
            CelestialBodyBundle::Star(bundle.clone()),
            MaterialMesh2dBundle {
                mesh,
                material,
                ..Default::default()
            },
        ));
        self.smi_dist.push(0.);
        self.sma_dist.push(0.);

        let star = self.galaxy.get_body(id).unwrap();
        (star.clone(), bundle.clone())
    }

    pub fn gen_planet(
        &mut self,
        star: &CelestialBody,
        system_id: usize,
        system_edge: f64,
    ) -> Result<(CelestialBody, CelestialBodyId), PlanetGenerationError> {
        let system = self.galaxy.get_system(system_id).unwrap();

        let mass = self.rng.sample(PlanetMassDistribution)
            * consts::EARTH_MASS
            * consts::PLANET_MASS_SCALE;

        let (ty, density) = {
            if mass / consts::EARTH_MASS > consts::GIANT_PLANET_MASS_THRESHOLD {
                let density = self.rng.sample(GiantPlanetDensityDistribution);
                if density > consts::ICE_GIANT_PLANET_DENSITY_THRESHOLD {
                    (PlanetType::IceGiant, density)
                } else {
                    (PlanetType::GasGiant, density)
                }
            } else {
                (
                    PlanetType::Rocky,
                    self.rng.sample(RockyPlanetDensityDistribution),
                )
            }
        };

        let radius = math::mass_to_radius(mass, density)
            * consts::EARTH_RADIUS
            * consts::PLANET_RADIUS_SCALE;

        let (min_smi_dist, max_smi_dist) = {
            if system.bodies.len() == 1 {
                (
                    star.radius() + radius * consts::BASE_PLANET_INTERV_COEFF,
                    star.radius() * consts::PLANET_TO_STAR_DIST_COEFF + radius,
                )
            } else {
                let last_planet_id = system.bodies.last().unwrap();
                let rhs = self.galaxy.get_body(*last_planet_id).unwrap();
                let d1 = physics::mass_acc_to_dist(mass, consts::PLANET_ACC_THRESHOLD);
                let d2 = physics::mass_acc_to_dist(rhs.mass(), consts::PLANET_ACC_THRESHOLD);
                let pos = self.smi_dist.last().unwrap();
                (
                    consts::BASE_PLANET_INTERV_COEFF * rhs.radius() + *pos,
                    consts::BASE_PLANET_INTERV_COEFF * rhs.radius() + *pos + d1.max(d2),
                )
            }
        };

        if min_smi_dist > system_edge {
            return Err(PlanetGenerationError::MaxSystemRadiusExceeded);
        }

        let smi_dist = self.rng.sample(PlanetSmiDistDistribution {
            min: min_smi_dist,
            max: max_smi_dist,
        });
        let sma_dist = smi_dist * self.rng.sample(self.cfg.pln_cfg.sma_smi_ratio);

        self.smi_dist.push(smi_dist);
        self.sma_dist.push(sma_dist);

        let init_vel = physics::vis_viva_get_smi_vel(star.mass() + mass, smi_dist, sma_dist);

        let body = CelestialBody::new(
            star.pos() + consts::DEFAULT_BODY_EXTEND_AXIS * smi_dist,
            radius,
            mass,
            init_vel * consts::DEFAULT_BODY_VEL_DIR,
        );

        let (id, systemic_id) = self.galaxy.add_body(system_id, body);

        let mesh = Mesh2dHandle(self.mesh_asstes.generate(self.meshes, id, radius));
        let (material, color) = self
            .material_assets
            .generate(self.materials, id, &mut self.rng);

        self.galaxy.set_color(id, color);

        let bundle = PlanetBundle {
            id,
            systemic_id,
            name: CelestialBodyName("".to_string()),
            ty,
            tag: Planet,
        };
        self.bundles.push((
            CelestialBodyBundle::Planet(bundle.clone()),
            MaterialMesh2dBundle {
                mesh,
                material,
                ..Default::default()
            },
        ));

        Ok((self.galaxy.get_body(id).unwrap().clone(), id))
    }

    fn gen_moon(
        &mut self,
        system_id: usize,
        star: &CelestialBody,
        planet_id: CelestialBodyId,
        planet: &CelestialBody,
    ) -> Result<(), MoonGenerationError> {
        let mass = self.rng.sample(MoonMassDistribution);
        let density = self.rng.sample(MoonDensityDistribution);
        let radius = math::mass_to_radius(mass, density) * consts::MOON_RADIUS_SCALE;
        let planet_smi_dist = self.smi_dist[planet_id.0];
        let smi_dist_incre_coeff = self.rng.sample(MoonSmiDistIncreCoeffDistribution);
        let smi_dist_rel = {
            if self.smi_dist.len() - 1 == planet_id.0 {
                planet.radius() * (consts::MOON_TO_PLANET_DIST_COEFF * smi_dist_incre_coeff).max(1.)
                    + radius
            } else {
                let last_moon_smi_dist = self.smi_dist.last().unwrap();
                *last_moon_smi_dist - planet_smi_dist
                    + radius
                    + planet.radius() * smi_dist_incre_coeff
            }
        };
        let sma_dist_rel = smi_dist_rel * self.rng.sample(self.cfg.moon_cfg.sma_smi_ratio);

        let a1 = physics::mass_dist_to_acc(star.mass(), planet_smi_dist);
        let a2 = physics::mass_acc_to_dist(planet.mass(), sma_dist_rel);
        if a1 * consts::MOON_GEN_STAR_PLANET_ACC_RATIO_THRESHOLD > a2 {
            return Err(MoonGenerationError::MinAccNotMet);
        }

        self.smi_dist.push(planet_smi_dist + smi_dist_rel);
        self.sma_dist.push(planet_smi_dist - sma_dist_rel);

        let init_vel =
            physics::vis_viva_get_smi_vel(planet.mass() + mass, smi_dist_rel, sma_dist_rel);

        let body = CelestialBody::new(
            star.pos() + (planet_smi_dist + smi_dist_rel) * consts::DEFAULT_BODY_EXTEND_AXIS,
            radius,
            mass,
            planet.vel() + init_vel * consts::DEFAULT_BODY_VEL_DIR,
        );

        let (id, systemic_id) = self.galaxy.add_body(system_id, body);

        let mesh = Mesh2dHandle(self.mesh_asstes.generate(self.meshes, id, radius));
        let (material, color) = self
            .material_assets
            .generate(self.materials, id, &mut self.rng);

        self.galaxy.set_color(id, color);

        self.bundles.push((
            CelestialBodyBundle::Moon(MoonBundle {
                id,
                systemic_id,
                name: CelestialBodyName("".to_string()),
                tag: Moon,
            }),
            MaterialMesh2dBundle {
                mesh,
                material,
                ..Default::default()
            },
        ));

        Ok(())
    }
}
