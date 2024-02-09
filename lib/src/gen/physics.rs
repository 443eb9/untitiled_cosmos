use crate::{consts, sim::resources::CelestialBody};

#[inline]
pub fn min_spd_to_dist(v: f64, center_mass: f64) -> f64 {
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
pub fn mass_radius_to_force(mass1: f64, mass2: f64, radius: f64) -> f64 {
    consts::G * mass1 * mass2 / radius / radius
}

#[inline]
pub fn mass_radius_to_acc(mass: f64, radius: f64) -> f64 {
    consts::G * mass / radius / radius
}

#[inline]
pub fn mass_acc_to_radius(mass: f64, acc: f64) -> f64 {
    (consts::G * mass / acc).sqrt()
}
