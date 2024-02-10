use crate::{consts, sim::resources::CelestialBody};

#[inline]
pub fn linear_spd_to_dist(v: f64, center_mass: f64) -> f64 {
    (consts::G * center_mass / v / v).sqrt()
}

#[inline]
pub fn vis_viva_get_smi_vel(total_mass: f64, dist: f64, smi_dist: f64) -> f64 {
    (consts::G * total_mass * (2. / dist - 1. / smi_dist)).sqrt()
}

#[inline]
pub fn force_between(lhs: &CelestialBody, rhs: &CelestialBody) -> f64 {
    consts::G * lhs.mass() * rhs.mass() / lhs.pos().distance_squared(rhs.pos())
}

#[inline]
pub fn mass_dist_to_force(m1: f64, m2: f64, d: f64) -> f64 {
    consts::G * m1 * m2 / d / d
}

#[inline]
pub fn mass_dist_to_acc(m: f64, d: f64) -> f64 {
    consts::G * m / d / d
}

#[inline]
pub fn mass_acc_to_dist(m: f64, a: f64) -> f64 {
    (consts::G * m / a).sqrt()
}
