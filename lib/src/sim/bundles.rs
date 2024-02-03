use bevy::ecs::bundle::Bundle;

use super::components::{CelestialBodyGalacticId, CelestialBodyId, CelestialBodyName, StarClass};

#[derive(Bundle, Clone)]
pub struct PlanetBundle {
    pub id: CelestialBodyId,
    pub galactic_id: CelestialBodyGalacticId,
    pub name: CelestialBodyName,
}

#[derive(Bundle, Clone)]
pub struct StarBundle {
    pub id: CelestialBodyId,
    pub galactic_id: CelestialBodyGalacticId,
    pub name: CelestialBodyName,
    pub star_class: StarClass,
}
