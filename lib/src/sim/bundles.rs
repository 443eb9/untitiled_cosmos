use bevy::ecs::bundle::Bundle;

use super::components::{CelestialBodyId, CelestialBodyName, CelestialBodySystemId, StarClass};

pub enum CelestialBodyBundle {
    Star(StarBundle),
    Planet(PlanetBundle),
}

#[derive(Bundle, Clone)]
pub struct StarBundle {
    pub id: CelestialBodyId,
    pub system_id: CelestialBodySystemId,
    pub name: CelestialBodyName,
    pub class: StarClass,
}

#[derive(Bundle, Clone)]
pub struct PlanetBundle {
    pub id: CelestialBodyId,
    pub galactic_id: CelestialBodySystemId,
    pub name: CelestialBodyName,
}
