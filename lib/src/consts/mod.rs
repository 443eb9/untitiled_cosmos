use std::ops::Range;

use bevy::math::DVec2;

use crate::sci::chemistry::Substance;

pub mod greek_alphabets;

pub const GLOBAL_CONFIG: &str = "cosmos/assets/config/global_config.json";
pub const STAR_PROPERTIES: &str = "cosmos/assets/config/star_properties.json";
pub const SUBSTANCE_ASSETS: &str = "cosmos/assets/config/substance_properties.json";
pub const SUBSTANCE_VISUAL_ASSETS: &str = "cosmos/assets/config/substance_visuals.json";
pub const STAR_NAMES: &str = "cosmos/assets/config/star_names.json";
pub const UNITS_INFO: &str = "cosmos/assets/config/units_info.json";
pub const BODY_STYLISH: &str = "cosmos/assets/config/body_stylish.json";

pub const DEFAULT_BODY_EXTEND_AXIS: DVec2 = DVec2::Y;
pub const DEFAULT_BODY_VEL_DIR: DVec2 = DVec2::X;

pub const SUN_MASS: f64 = 1.989e30;
pub const SUN_RADIUS: f64 = 6.96342e8;
pub const SUN_LUMINOSITY: f64 = 3.828e26;

pub const LY: f64 = 9.4605284e15;

pub const EARTH_MASS: f64 = 5.972e24;
pub const EARTH_RADIUS: f64 = 6.371e6;
pub const STANDARD_ATMOSPHERE_DENSITY: f64 = 1.225e-3;

/// Describes the minimum distance between a star and a planet.
///
/// `min_dist = r_star * PLANET_TO_STAR_DIST_COEFF + r_planet`
pub const PLANET_TO_STAR_DIST_COEFF: f64 = 5.;
/// Describes the minimum distance between two planets.
///
/// This should be multiplied by the last planet's radius.
pub const BASE_PLANET_INTERV_COEFF: f64 = 8.;
/// The minimun ratio between the acceleration of a moon to the planet and to the star.
///
/// This prevents from the moon to be too close to the star and fail to orbit the planet.
pub const MOON_GEN_STAR_PLANET_ACC_RATIO_THRESHOLD: f64 = 0.5;
pub const MIN_MOON_DIST_TO_PLANET_COEFF: f64 = 1.2;
pub const MAX_MOON_DIST_TO_PLANET_COEFF: f64 = 10.;

pub const CELESTIAL_SIM_STEP: f64 = 0.01;

pub const SQART_2_PI: f64 = 2.5066282746310005024157652848110452530069867406099383166299235763;

pub const G: f64 = 6.67430e-11;
pub const STEFAN_BOLTZMANN: f64 = 5.670374419e-8;
pub const IDEAL_GAS_CONST: f64 = 8.31446261815324;

pub const STAR_ACC_THRESHOLD: f64 = 1e-4;
pub const PLANET_ACC_THRESHOLD: f64 = 1e0;

/// In earth masses.
pub const GIANT_PLANET_MASS_THRESHOLD: f64 = 1e1;
pub const ICE_GIANT_PLANET_DENSITY_THRESHOLD: f64 = 1.5;

pub const STAR_MASS_SCALE: f64 = 2e-7;
pub const PLANET_MASS_SCALE: f64 = 1e-5;
pub const MOON_MASS_SCALE: f64 = 1e-15;

pub const STAR_RADIUS_SCALE: f64 = 1e-5;
pub const PLANET_RADIUS_SCALE: f64 = 2e-11;
pub const MOON_RADIUS_SCALE: f64 = 1e-4;

pub const PLANET_EFFCETIVE_TEMP_SCALE: f64 = 0.2;

pub const COMPOSITION_MIN: f64 = 0.05;
pub const ATMOSPHERE_DENSITY_MIN: f64 = 1e-3;
pub const STAR_COMPOSITION: [(Substance, Range<f64>); 2] = [
    (Substance::Hydrogen, 0.7..0.9),
    (Substance::Helium, 0.1..0.3),
];
pub const GAS_GIANT_ATMO: [(Substance, Range<f64>); 2] = [
    (Substance::Hydrogen, 0.7..0.9),
    (Substance::Helium, 0.1..0.3),
];
pub const ICE_GIANT_ATMO: [(Substance, Range<f64>); 7] = [
    (Substance::Hydrogen, 0.2..0.4),
    (Substance::Helium, 0.1..0.3),
    (Substance::Methane, 0f64..0.5),
    (Substance::Water, 0f64..0.5),
    (Substance::Ammonia, 0f64..0.3),
    (Substance::AmorphousIce, 0f64..0.2),
    (Substance::HydrogenSulfide, 0f64..0.3),
];
pub const ROCKY_PLANET_CRUST: [(Substance, Range<f64>); 7] = [
    (Substance::SiliconDioxide, 0.3..0.9),
    (Substance::AluminumOxide, 0f64..0.3),
    (Substance::FerricOxide, 0f64..0.3),
    (Substance::CalciumCarbonate, 0f64..0.6),
    (Substance::Water, 0f64..0.3),
    (Substance::AmorphousIce, 0f64..0.2),
    (Substance::SulfuricAcid, 0f64..0.3),
];
pub const ROCKY_PLANET_ATMO: [(Substance, Range<f64>); 6] = [
    (Substance::Nitrogen, 0.1..0.9),
    (Substance::Oxygen, 0f64..0.4),
    (Substance::CarbonDioxide, 0f64..0.2),
    (Substance::Ammonia, 0f64..0.8),
    (Substance::SulfurDioxide, 0f64..0.5),
    (Substance::Phosphine, 0f64..0.1),
];

pub const PRE_SIM_STEPS: usize = 10000;
