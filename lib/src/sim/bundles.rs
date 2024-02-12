use bevy::ecs::bundle::Bundle;

use super::components::{
    CelestialBodyAtmosphere, CelestialBodyColor, CelestialBodyCrust, CelestialBodyEffectiveTemp,
    CelestialBodyId, CelestialBodyName, CelestialBodySubstanceProps, Moon, Planet, PlanetType,
    Star, StarClass, StarLuminosity,
};

pub enum CelestialBodyBundle {
    Star(StarBundle),
    Planet {
        planet: PlanetBundle,
        crust: Option<CelestialBodyCrust>,
        atmo: Option<CelestialBodyAtmosphere>,
    },
    Moon {
        moon: MoonBundle,
        crust: CelestialBodyCrust,
        atmo: Option<CelestialBodyAtmosphere>,
    },
}

#[derive(Bundle, Clone)]
pub struct StarBundle {
    pub id: CelestialBodyId,
    pub color: CelestialBodyColor,
    pub name: CelestialBodyName,
    pub class: StarClass,
    pub composition: CelestialBodyCrust,
    pub effective_temp: CelestialBodyEffectiveTemp,
    pub luminosity: StarLuminosity,
    pub tag: Star,
}

#[derive(Bundle, Clone)]
pub struct PlanetBundle {
    pub id: CelestialBodyId,
    pub color: CelestialBodyColor,
    pub name: CelestialBodyName,
    pub effective_temp: CelestialBodyEffectiveTemp,
    pub substance_props: CelestialBodySubstanceProps,
    pub ty: PlanetType,
    pub tag: Planet,
}

#[derive(Bundle, Clone)]
pub struct MoonBundle {
    pub id: CelestialBodyId,
    pub color: CelestialBodyColor,
    pub name: CelestialBodyName,
    pub effective_temp: CelestialBodyEffectiveTemp,
    pub substance_props: CelestialBodySubstanceProps,
    pub tag: Moon,
}
