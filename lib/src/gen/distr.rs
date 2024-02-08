use std::f64::consts::PI;

use bevy::math::DVec2;
use rand::Rng;
use rand_distr::Distribution;

#[inline]
pub fn univ_pdf() -> impl Fn(f64) -> f64 {
    move |x| (1. / (500. * x)).min(1.)
}

#[inline]
pub fn stellar_density_pdf(galaxy_radius: f64) -> impl Fn(f64) -> f64 {
    move |sqr_radius| 1. - (sqr_radius / galaxy_radius).powi(5)
}

#[derive(Clone, Copy)]
pub struct StarPosDistribution {
    galaxy_radius: f64,
}

impl StarPosDistribution {
    pub fn new(galaxy_radius: f64) -> Self {
        Self { galaxy_radius }
    }
}

impl Distribution<DVec2> for StarPosDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DVec2 {
        let r = rng.gen_range(0f64..self.galaxy_radius);
        let theta = rng.gen_range(0f64..PI * 2.);
        let (s_theta, c_theta) = theta.sin_cos();
        DVec2::new(r * c_theta, r * s_theta)
    }
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

pub struct GalaxyStarDensityDistribution {
    galaxy_radius: f64,
}

impl GalaxyStarDensityDistribution {
    const PDF_MAX: f64 = 1.;

    pub fn new(galaxy_radius: f64) -> Self {
        Self { galaxy_radius }
    }
}

impl Distribution<DVec2> for GalaxyStarDensityDistribution {
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

#[cfg(test)]
mod test {
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
}
