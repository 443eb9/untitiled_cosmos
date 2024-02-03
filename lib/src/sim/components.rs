use bevy::ecs::component::Component;

#[derive(Component, Debug, Clone)]
pub struct CelestialBodyId(pub usize);

#[derive(Component, Debug, Clone)]
pub struct CelestialBodyGalacticId {
    pub id: usize,
    pub galaxy_id: usize,
}

#[derive(Component, Debug, Clone)]
pub struct CelestialBodyName(pub String);

#[derive(Component, Debug, Clone)]
pub struct Planet;

#[derive(Component, Debug, Clone)]
pub struct Star;

#[derive(Component, Debug, Clone)]
pub enum StarClass {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}

#[derive(Component, Debug, Clone)]
pub struct Moon;
