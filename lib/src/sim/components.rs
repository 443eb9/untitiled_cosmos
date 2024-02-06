use bevy::ecs::component::Component;

#[cfg(feature = "debug")]
use bevy::reflect::Reflect;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub struct CelestialBodyId(pub usize);

#[derive(Component, Clone, Copy)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub struct CelestialBodySystemId {
    pub in_system_id: usize,
    pub system_id: usize,
}

#[derive(Component, Clone)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub struct CelestialBodyName(pub String);

#[derive(Component, Clone)]
pub struct Planet;

#[derive(Component, Clone)]
pub struct Star;

#[derive(Component, Clone)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub enum StarClass {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}

#[derive(Component, Clone)]
pub struct Moon;
