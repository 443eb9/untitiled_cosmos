use bevy::ecs::bundle::Bundle;

use super::components::{CelestialBodyId, CelestialBodyName, CelestialBodySystemId, StarClass};

#[derive(Bundle, Clone)]
pub struct PlanetBundle {
    pub id: CelestialBodyId,
    pub galactic_id: CelestialBodySystemId,
    pub name: CelestialBodyName,
}

#[derive(Bundle, Clone)]
pub struct StarBundle {
    pub id: CelestialBodyId,
    pub system_id: CelestialBodySystemId,
    pub name: CelestialBodyName,
    pub star_class: StarClass,
}
