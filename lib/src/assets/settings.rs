use bevy::{ecs::system::Resource, render::color::Color, utils::HashMap};
use serde::{de::Visitor, Deserialize};

use crate::{
    consts,
    math::{Unit, UnitClass},
    sim::components::StarClass,
    utils,
};

pub struct PackedStarInfo {
    pub class: StarClass,
    pub mass: f64,
    pub radius: f64,
    pub luminosity: f64,
    pub temperature: f64,
    pub color: Color,
}

impl<'de> Deserialize<'de> for PackedStarInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PackedStarInfoVisitor;
        impl<'de> Visitor<'de> for PackedStarInfoVisitor {
            type Value = PackedStarInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map with mass, radius, luminosity, temperature, and color")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                #[derive(Deserialize)]
                struct VecColor {
                    r: f32,
                    g: f32,
                    b: f32,
                    a: f32,
                }
                impl Into<Color> for VecColor {
                    fn into(self) -> Color {
                        Color::rgba(self.r, self.g, self.b, self.a)
                    }
                }
                let mut star_class = None;
                let mut mass = None;
                let mut radius = None;
                let mut luminosity = None;
                let mut temperature = None;
                let mut color = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "star_class" => star_class = Some(map.next_value()?),
                        "mass" => mass = Some(map.next_value()?),
                        "radius" => radius = Some(map.next_value()?),
                        "luminosity" => luminosity = Some(map.next_value()?),
                        "temperature" => temperature = Some(map.next_value()?),
                        "color" => color = Some(map.next_value::<VecColor>()?.into()),
                        _ => {}
                    }
                }

                Ok(PackedStarInfo {
                    class: star_class
                        .ok_or_else(|| serde::de::Error::missing_field("star_class"))?,
                    mass: mass.ok_or_else(|| serde::de::Error::missing_field("mass"))?,
                    radius: radius.ok_or_else(|| serde::de::Error::missing_field("radius"))?,
                    luminosity: luminosity
                        .ok_or_else(|| serde::de::Error::missing_field("luminosity"))?,
                    temperature: temperature
                        .ok_or_else(|| serde::de::Error::missing_field("temperature"))?,
                    color: color.ok_or_else(|| serde::de::Error::missing_field("color"))?,
                })
            }
        }

        deserializer.deserialize_map(PackedStarInfoVisitor)
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
