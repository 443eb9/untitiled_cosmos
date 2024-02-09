use std::cmp::Ordering;

use bevy::ecs::component::Component;

#[cfg(feature = "debug")]
use bevy::reflect::Reflect;
use serde_derive::Deserialize;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub struct CelestialBodyId(pub usize);

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub struct StarClass {
    pub ty: SpectralType,
    pub sub_ty: u8,
}

impl PartialOrd for StarClass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StarClass {
    fn cmp(&self, other: &Self) -> Ordering {
        let t = (self.ty as u8).cmp(&(other.ty as u8));
        if t == Ordering::Equal {
            self.sub_ty.cmp(&other.sub_ty)
        } else {
            t
        }
    }
}

impl StarClass {
    #[inline]
    pub fn to_index(self) -> usize {
        (self.ty as usize)
            .checked_sub(1)
            .unwrap_or(self.sub_ty as usize + 10 - 3 % 10)
            * 10
            + self.sub_ty as usize
            + 7
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub enum SpectralType {
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
