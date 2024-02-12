use bevy::{ecs::system::Resource, utils::HashMap};
use serde::Deserialize;

use crate::{
    consts,
    math::{self, HexRgbaColor, Unit, UnitClass},
    sim::components::StarClass,
    utils,
};

#[derive(Deserialize)]
pub struct PackedStarInfo {
    pub class: StarClass,
    pub mass: f64,
    pub radius: f64,
    pub luminosity: f64,
    pub effective_temp: f64,
    pub color: HexRgbaColor,
}

impl PackedStarInfo {
    pub fn lerp(&self, other: &Self, t: f64) -> PackedStarInfo {
        PackedStarInfo {
            class: self.class,
            mass: math::lerpf64(self.mass, other.mass, t),
            radius: math::lerpf64(self.radius, other.radius, t),
            luminosity: math::lerpf64(self.luminosity, other.luminosity, t),
            effective_temp: math::lerpf64(self.effective_temp, other.effective_temp, t),
            color: self.color.lerp(other.color, t as f32),
        }
    }
}

#[derive(Resource)]
pub struct StarProperties(Vec<PackedStarInfo>);

impl Default for StarProperties {
    fn default() -> Self {
        Self(utils::deser(consts::STAR_PROPERTIES).unwrap())
    }
}

impl StarProperties {
    #[inline]
    pub fn get(&self, class: StarClass) -> &PackedStarInfo {
        self.0.get(class.to_index()).unwrap()
    }

    #[inline]
    pub fn get_at(&self, index: usize) -> &PackedStarInfo {
        self.0.get(index).unwrap()
    }

    #[inline]
    pub fn find_bound(
        &self,
        value: f64,
        key: impl Fn(&PackedStarInfo) -> f64,
    ) -> (&PackedStarInfo, &PackedStarInfo) {
        for i in 0..self.0.len() - 1 {
            let min = self.0.get(i).unwrap();
            if key(min) <= value {
                let max = self.0.get(i - 1).unwrap();
                return (min, max);
            }
        }

        panic!("Invalid value: {}", value)
    }

    #[inline]
    pub fn raw(&self) -> &Vec<PackedStarInfo> {
        &self.0
    }
}

#[derive(Resource)]
pub struct ConstellationNames(Vec<String>);

impl Default for ConstellationNames {
    fn default() -> Self {
        Self(utils::deser(consts::STAR_NAMES).unwrap())
    }
}

impl ConstellationNames {
    #[inline]
    pub fn get(&self, index: usize) -> &str {
        &self.0[index]
    }

    #[inline]
    pub fn get_cloned(&self, index: usize) -> String {
        self.0[index].clone()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Resource)]
pub struct UnitsInfo(HashMap<UnitClass, Unit>);

impl Default for UnitsInfo {
    fn default() -> Self {
        Self(utils::deser(consts::UNITS_INFO).unwrap())
    }
}

impl UnitsInfo {
    #[inline]
    pub fn get(&self, class: UnitClass) -> &Unit {
        self.0.get(&class).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_bound() {
        let props =
            utils::deser::<Vec<PackedStarInfo>>("../cosmos/assets/config/star_properties.json")
                .unwrap();
        let props = StarProperties(props);
        let (min, max) = props.find_bound(2.5, |info| info.mass);
        assert_eq!(min.mass, 2.18);
        assert_eq!(max.mass, 2.75);
    }
}
