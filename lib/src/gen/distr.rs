use bevy::math::DVec2;
use rand::Rng;
use rand_distr::Distribution;

// TODO read data from SETTINGS.md

#[inline]
pub fn univ_pdf() -> impl Fn(f64) -> f64 {
    move |x| (1. / (200. * x)).min(1.) - 0.02
}

#[inline]
pub fn stellar_density_pdf(galaxy_radius: f64) -> impl Fn(f64) -> f64 {
    move |sqr_radius| 1. - (sqr_radius / galaxy_radius).powi(5)
}

pub struct StarMassDistribution;

impl StarMassDistribution {
    const MIN: f64 = 0.;
    const MAX: f64 = 120.;
    const PDF_MAX: f64 = 0.033097632263427452873193360684310729;
    const SUN_MASS: f64 = 1.9891e30;
}

impl Distribution<f64> for StarMassDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        loop {
            let x = rng.gen_range(0f64..1f64);
            let y = rng.gen_range(0f64..Self::PDF_MAX);
            if y < univ_pdf()(x) {
                return (x * (Self::MAX - Self::MIN) + Self::MIN) * Self::SUN_MASS;
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
    fn test_mass_distr() {
        let mut records = vec![0; 120];
        for _ in 0..64 {
            let mut rng = rand::thread_rng();
            let mass = StarMassDistribution.sample(&mut rng);
            records[(mass / StarMassDistribution::SUN_MASS) as usize] += 1;
        }
        for (i, &count) in records.iter().enumerate() {
            println!("{}: {}", i, count);
        }
    }
}
