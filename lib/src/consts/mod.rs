use bevy::math::DVec2;

pub mod greek_alphabets;

pub const GLOBAL_CONFIG: &str = "cosmos/assets/config/global_config.json";
pub const STAR_PROPERTIES: &str = "cosmos/assets/config/star_properties.json";
pub const UNITS_INFO: &str = "cosmos/assets/config/units_info.json";

pub const DEFAULT_BODY_EXTEND_AXIS: DVec2 = DVec2::Y;
pub const DEFAULT_BODY_VEL_DIR: DVec2 = DVec2::X;

pub const SUN_MASS: f64 = 1.989e30;
pub const SUN_RADIUS: f64 = 6.96342e8;
pub const SUN_LUMINOSITY: f64 = 3.828e26;

pub const EARTH_MASS: f64 = 5.972e24;
pub const EARTH_RADIUS: f64 = 6.371e6;

/// Describes the minimum distance between a star and a planet.
/// 
/// `min_dist = r_star * PLANET_TO_STAR_DIST_COEFF + r_planet`
pub const PLANET_TO_STAR_DIST_COEFF: f64 = 2.;
pub const MOON_TO_PLANET_DIST_COEFF: f64 = 1.8;
pub const BASE_PLANET_INTERV_COEFF: f64 = 10.;
pub const MOON_GEN_STAR_PLANET_ACC_RATIO_THRESHOLD: f64 = 0.5;

pub const FIXED_STEP: f64 = 0.02;

pub const SQART_2_PI: f64 = 2.5066282746310005024157652848110452530069867406099383166299235763;

pub const G: f64 = 6.67430e-11;

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
pub const MOON_RADIUS_SCALE: f64 = 1e-5;

pub const LY: f64 = 9.4605284e15;
