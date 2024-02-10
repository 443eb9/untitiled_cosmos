use bevy::ecs::bundle::Bundle;

use super::components::{
    CelestialBodyId, CelestialBodyName, CelestialBodySystemId, Moon, Planet, PlanetType, StarClass,
};

pub enum CelestialBodyBundle {
    Star(StarBundle),
    Planet(PlanetBundle),
    Moon(MoonBundle),
}

#[derive(Bundle, Clone)]
pub struct StarBundle {
    pub id: CelestialBodyId,
    pub systemic_id: CelestialBodySystemId,
    pub name: CelestialBodyName,
    pub class: StarClass,
}

#[derive(Bundle, Clone)]
pub struct PlanetBundle {
    pub id: CelestialBodyId,
    pub systemic_id: CelestialBodySystemId,
    pub name: CelestialBodyName,
    pub ty: PlanetType,
    pub tag: Planet,
}

#[derive(Bundle, Clone)]
pub struct MoonBundle {
    pub id: CelestialBodyId,
    pub systemic_id: CelestialBodySystemId,
    pub name: CelestialBodyName,
    pub tag: Moon,
}
