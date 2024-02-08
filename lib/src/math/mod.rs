use serde_derive::Deserialize;

pub mod aabbs;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum UnitClass {
    MassAstro,
    RadiusAstro,
    LuminosityAstro,
    TemperatureAstro,
}

#[derive(Clone, Deserialize)]
pub struct Unit {
    pub name: String,
    pub symbol: String,
}
