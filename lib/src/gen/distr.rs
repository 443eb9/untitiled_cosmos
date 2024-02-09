use bevy::math::DVec2;
use rand::Rng;
use rand_distr::{Distribution, Normal, Uniform};

use crate::consts;

use super::physics;

#[inline]
pub fn univ_pdf() -> impl Fn(f64) -> f64 {
    move |x| (1. / (500. * x)).min(1.)
}

#[inline]
pub fn stellar_density_pdf(galaxy_radius: f64) -> impl Fn(f64) -> f64 {
    move |sqr_radius| 1. - (sqr_radius / galaxy_radius).powi(5)
}

#[inline]
pub fn planet_smi_dist_incre_pdf() -> impl Fn(f64) -> f64 {
    move |x| (x.powf(-x) - 1. / (100. * x)).powi(15)
}

#[inline]
pub fn planet_mass_pdf() -> impl Fn(f64) -> f64 {
    move |x| {
        2. * (-(150. * x).powi(5) + 20.).max(0.)
            + ((300. * x - 20.).tanh() - (6. * x - 1.).tanh()) * 0.2
    }
}

#[inline]
pub fn max_num_planets(spectral_index: usize, distr: Uniform<f32>, rng: &mut impl Rng) -> usize {
    let x = spectral_index as f32;
    let t = (8. / (1. + ((x - 33.) / 12.).exp()) + 3.).floor();
    (rng.sample(distr) * t) as usize
}

#[derive(Clone, Copy)]
pub struct StarMassDistribution;

impl StarMassDistribution {
    const MIN: f64 = 0.078;
    const MAX: f64 = 120.;
    const PDF_MAX: f64 = 4.;
}

impl Distribution<f64> for StarMassDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        loop {
            let x = rng.gen_range(0f64..1f64);
            let y = rng.gen_range(0f64..Self::PDF_MAX);
            if y < univ_pdf()(x) {
                return x * (Self::MAX - Self::MIN) + Self::MIN;
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct StarPosDistribution {
    galaxy_radius: f64,
}

impl StarPosDistribution {
    const PDF_MAX: f64 = 1.;

    pub fn new(galaxy_radius: f64) -> Self {
        Self { galaxy_radius }
    }
}

impl Distribution<DVec2> for StarPosDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DVec2 {
        loop {
            let x = rng.gen_range(0f64..1f64);
            let y = rng.gen_range(0f64..Self::PDF_MAX);
            let sqr_radius = x * x + y * y;
            if sqr_radius < 1. {
                return DVec2::new(x, y) * self.galaxy_radius;
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct PlanetMassDistribution;

impl PlanetMassDistribution {
    pub const MIN: f64 = 0.02;
    pub const MAX: f64 = 300.;
    pub const PDF_MAX: f64 = 19.952;
}

impl Distribution<f64> for PlanetMassDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        loop {
            let x = rng.gen_range(0f64..1f64);
            let y = rng.gen_range(0f64..Self::PDF_MAX);
            if y < planet_mass_pdf()(x) {
                return x * (Self::MAX - Self::MIN) + Self::MIN;
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct PlanetSmiDistDistribution {
    pub min: f64,
    pub max: f64,
}

impl PlanetSmiDistDistribution {
    pub const PDF_MAX: f64 = 0.0693;
}

impl Distribution<f64> for PlanetSmiDistDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        loop {
            let x = rng.gen_range(0f64..1f64);
            let y = rng.gen_range(0f64..Self::PDF_MAX);
            if y < planet_smi_dist_incre_pdf()(x) {
                return x * (self.max - self.min) + self.min;
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct GasGiantDensityDistribution;

impl GasGiantDensityDistribution {
    pub const MIN: f64 = 0.3;
    pub const MAX: f64 = 1.1;
}

impl Distribution<f64> for GasGiantDensityDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let x = rng.sample(Normal::new(0.5, 0.1).unwrap());
        x * (Self::MAX - Self::MIN) + Self::MIN
    }
}

#[cfg(test)]
mod test {
    use crate::gen::GalaxyGeneratorConfig;

    use super::*;

    #[test]
    fn test_smd() {
        let mut records = vec![0; 120];
        for _ in 0..64 {
            let mut rng = rand::thread_rng();
            let mass = StarMassDistribution.sample(&mut rng);
            records[mass as usize] += 1;
        }
        println!("{:?}", records);
    }

    #[test]
    fn test_max_planet_count() {
        for i in 0..67 {
            println!(
                "{}",
                max_num_planets(
                    i,
                    GalaxyGeneratorConfig::new_debug().pln_cfg.num_coeff,
                    &mut rand::thread_rng()
                )
            );
        }
    }
}
