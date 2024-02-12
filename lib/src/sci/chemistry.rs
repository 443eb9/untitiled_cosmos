use std::fmt::{Display, Formatter, Result};

use bevy::{render::color::Color, utils::HashMap};
use serde_derive::Deserialize;

#[cfg(feature = "debug")]
use bevy::reflect::Reflect;

use crate::{assets::SubstanceAssets, consts, math::HexRgbaColor};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MatterState {
    Solid,
    Liquid,
    Gas,
}

#[derive(Clone, Deserialize)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub struct SubstanceProperty {
    pub melting_point: f64,
    pub boiling_point: f64,
    pub heat_of_vaporization: f64,
    pub vapor: Option<Substance>,
    pub color: Vec<HexRgbaColor>,
}

impl SubstanceProperty {
    #[inline]
    pub fn get_boiling_point_at(&self, density: f64) -> f64 {
        1. / (1. / self.boiling_point
            - (consts::IDEAL_GAS_CONST * (density / consts::STANDARD_ATMOSPHERE_DENSITY).ln()
                / self.heat_of_vaporization))
    }

    #[inline]
    pub fn get_hex_color_at(&self, state: MatterState) -> HexRgbaColor {
        self.color[state as usize]
    }

    #[inline]
    pub fn get_color_at(&self, state: MatterState) -> Color {
        self.color[state as usize].into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[cfg_attr(feature = "debug", derive(Reflect))]
pub enum Substance {
    Hydrogen,
    Helium,
    Oxygen,
    Ammonia,
    Methane,
    CarbonDioxide,
    Nitrogen,
    SulfurDioxide,
    SulfuricAcid,
    Phosphine,
    Silane,
    HydrogenSulfide,

    AmorphousIce,
    Water,

    SiliconDioxide,
    FerricOxide,
    AluminumOxide,
    CalciumCarbonate,
    Coal,
}

impl Display for Substance {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Substance {
    pub fn is_greenhouse_gas(&self) -> bool {
        matches!(
            self,
            Substance::Water
                | Substance::CarbonDioxide
                | Substance::Methane
                | Substance::Ammonia
                | Substance::SulfurDioxide
                | Substance::Phosphine
        )
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub struct SubstanceContent(HashMap<Substance, f64>);

impl SubstanceContent {
    pub fn new(composition: HashMap<Substance, f64>) -> Self {
        Self(composition)
    }

    #[inline]
    pub fn get_content(&self, compound: Substance) -> Option<f64> {
        self.0.get(&compound).cloned()
    }

    #[inline]
    pub fn contains(&self, compound: Substance) -> bool {
        self.0.contains_key(&compound)
    }

    #[inline]
    pub fn num_compounds(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&Substance, &f64)> {
        self.0.iter()
    }

    #[inline]
    pub fn normalize(&mut self) {
        let sum: f64 = self.0.values().sum();
        self.0.iter_mut().for_each(|(_, v)| *v /= sum);
    }

    #[inline]
    pub fn normalized(mut self) -> Self {
        self.normalize();
        self
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut HashMap<Substance, f64> {
        &mut self.0
    }

    #[inline]
    pub fn estimate_color(&self, props: &SubstanceAssets, state: MatterState) -> HexRgbaColor {
        let mut color = HexRgbaColor::new(0., 0., 0., 0.);
        self.0.iter().for_each(|(sub, content)| {
            let sub_col = props.get(*sub).get_hex_color_at(state);
            color = color.lerp(sub_col, *content as f32);
        });
        color
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_melting_point() {
        let data = r#"
        {
            "name": "Water",
            "melting_point": 273.15,
            "boiling_point": 373.13,
            "heat_of_vaporization": 40650
        }
        "#;
        let prop: SubstanceProperty = serde_json::from_str(data).unwrap();
        let mp = prop.get_boiling_point_at(1.5 * consts::STANDARD_ATMOSPHERE_DENSITY);
        println!("{}", mp - 273.15);
    }
}
