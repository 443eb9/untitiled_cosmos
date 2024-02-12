use std::f64::consts::PI;

use bevy::render::color::Color;
use serde::{de::Visitor, Deserialize};

#[cfg(feature = "debug")]
use bevy::reflect::Reflect;

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

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug", derive(Reflect, Debug))]
pub struct HexRgbaColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl<'de> Deserialize<'de> for HexRgbaColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct HexColorVisitor;
        impl<'de> Visitor<'de> for HexColorVisitor {
            type Value = HexRgbaColor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a hex color string like #RRGGBB or #RRGGBBAA")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let v = v.trim_start_matches('#');
                let r = u8::from_str_radix(&v[0..2], 16).map_err(serde::de::Error::custom)?;
                let g = u8::from_str_radix(&v[2..4], 16).map_err(serde::de::Error::custom)?;
                let b = u8::from_str_radix(&v[4..6], 16).map_err(serde::de::Error::custom)?;
                let a = if v.len() == 8 {
                    u8::from_str_radix(&v[6..8], 16).map_err(serde::de::Error::custom)?
                } else {
                    255
                };
                Ok(HexRgbaColor {
                    r: r as f32 / 255.,
                    g: g as f32 / 255.,
                    b: b as f32 / 255.,
                    a: a as f32 / 255.,
                })
            }
        }
        deserializer.deserialize_str(HexColorVisitor)
    }
}

impl Into<Color> for HexRgbaColor {
    fn into(self) -> Color {
        Color::rgba(self.r, self.g, self.b, self.a)
    }
}

impl HexRgbaColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        HexRgbaColor { r, g, b, a }
    }

    pub fn lerp(self, other: HexRgbaColor, t: f32) -> HexRgbaColor {
        HexRgbaColor {
            r: lerpf32(self.r, other.r, t),
            g: lerpf32(self.g, other.g, t),
            b: lerpf32(self.b, other.b, t),
            a: lerpf32(self.a, other.a, t),
        }
    }
}

#[inline]
pub fn lerpf64(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

#[inline]
pub fn lerpf32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[inline]
pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::rgba(
        lerpf32(a.r(), b.r(), t),
        lerpf32(a.g(), b.g(), t),
        lerpf32(a.b(), b.b(), t),
        lerpf32(a.a(), b.a(), t),
    )
}

#[inline]
pub fn mass_to_radius(mass: f64, density: f64) -> f64 {
    (3. * mass / (4. * PI * density)).cbrt()
}

#[inline]
pub fn volume_to_radius(volume: f64) -> f64 {
    (3. * volume / (4. * PI)).cbrt()
}

#[inline]
pub fn mass_radius_to_density(mass: f64, radius: f64) -> f64 {
    3. * mass / (4. * PI * radius * radius * radius)
}
