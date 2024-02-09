use std::f64::consts::PI;

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

#[inline]
pub fn mass_to_radius(mass: f64, density: f64) -> f64 {
    (3. * mass / (4. * PI * density)).cbrt()
}

#[inline]
pub fn volume_to_radius(volume: f64) -> f64 {
    (3. * volume / (4. * PI)).cbrt()
}
