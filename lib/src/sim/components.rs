use std::cmp::Ordering;

use crate::sci::chemistry::{Substance, SubstanceContent, SubstanceProperty};
use bevy::{ecs::component::Component, render::color::Color};

#[cfg(feature = "debug")]
use bevy::reflect::Reflect;
use serde_derive::Deserialize;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub struct CelestialBodyId(pub usize);

#[derive(Component, Clone)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub struct CelestialBodyName(pub String);

#[derive(Component, Clone, Copy)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub struct CelestialBodyColor(pub Color);

#[derive(Component, Clone)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub struct CelestialBodyCrust {
    pub content: SubstanceContent,
    pub density: f64,
}

#[derive(Component, Clone)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub struct CelestialBodyAtmosphere {
    pub content: SubstanceContent,
    pub density: f64,
}

#[derive(Component, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub struct CelestialBodyEffectiveTemp(pub f64);

#[derive(Component, Clone)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub struct CelestialBodySubstanceProps(Vec<SubstanceProperty>);

impl CelestialBodySubstanceProps {
    pub fn new(props: Vec<SubstanceProperty>) -> Self {
        Self(props)
    }

    #[inline]
    pub fn get(&self, substance: Substance) -> Option<&SubstanceProperty> {
        self.0.get(substance as usize)
    }
}

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

#[derive(Component, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub struct StarLuminosity(pub f64);

#[derive(Component, Clone)]
pub struct Planet;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub enum PlanetType {
    GasGiant,
    IceGiant,
    Rocky,
}

#[derive(Component, Clone)]
pub struct Moon;
