use bevy::math::DVec2;
use rand::Rng;
use rand_distr::{Distribution, Normal, Uniform};

use crate::consts;

#[inline]
pub fn univ_pdf() -> impl Fn(f64) -> f64 {
    move |x| (1. / (500. * x)).min(1.)
}

#[inline]
pub fn stellar_density_pdf(galaxy_radius: f64) -> impl Fn(f64) -> f64 {
    move |sqr_radius| 1. - (sqr_radius / galaxy_radius).powi(5)
}

#[inline]
pub fn planet_smi_pdf() -> impl Fn(f64) -> f64 {
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
pub fn moon_mass_pdf() -> impl Fn(f64) -> f64 {
    move |x| 26f64.powf(-50. * x) + 21f64.powf(-1.5 * (x + 1.))
}

#[inline]
pub fn moon_smi_dist_incre_coeff_pdf() -> impl Fn(f64) -> f64 {
    move |x| -(2. * (x - 0.5)).powi(2) + 1.
}

#[inline]
pub fn max_num_planets(spectral_index: usize, distr: Uniform<f32>, rng: &mut impl Rng) -> usize {
    let x = spectral_index as f32;
    let t = (8. / (1. + ((x - 33.) / 12.).exp()) + 3.).floor();
    (rng.sample(distr) * t) as usize
}

#[inline]
pub fn max_num_moons(mass: f64, distr: Uniform<f32>, rng: &mut impl Rng) -> usize {
    let mass = (mass / consts::EARTH_MASS / consts::PLANET_MASS_SCALE) as f32;
    let num_coeff = rng.sample(distr);
    let ty_coeff = (0.5 * mass).sqrt() / 3. + 0.9;
    (num_coeff * ty_coeff).round() as usize
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
        let x_distr = Uniform::new(0., 1.);
        let y_distr = Uniform::new(0., Self::PDF_MAX);

        loop {
            let x = rng.sample(x_distr);
            let y = rng.sample(y_distr);
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
        let x_distr = Uniform::new(0., 1.);
        let y_distr = Uniform::new(0., Self::PDF_MAX);

        loop {
            let x = rng.sample(x_distr);
            let y = rng.sample(y_distr);
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
        let x_distr = Uniform::new(0., 1.);
        let y_distr = Uniform::new(0., Self::PDF_MAX);

        loop {
            let x = rng.sample(x_distr);
            let y = rng.sample(y_distr);
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
        let x_distr = Uniform::new(0., 1.);
        let y_distr = Uniform::new(0., Self::PDF_MAX);

        loop {
            let x = rng.sample(x_distr);
            let y = rng.sample(y_distr);
            if y < planet_smi_pdf()(x) {
                return x * (self.max - self.min) + self.min;
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct RockyPlanetDensityDistribution;

impl RockyPlanetDensityDistribution {
    pub const MIN: f64 = 2.5;
    pub const MAX: f64 = 5.5;
}

impl Distribution<f64> for RockyPlanetDensityDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let x = rng
            .sample(Normal::<f64>::new(0.5, 0.25).unwrap())
            .clamp(0., 1.);
        x * (Self::MAX - Self::MIN) + Self::MIN
    }
}

#[derive(Clone, Copy)]
pub struct GiantPlanetDensityDistribution;

impl GiantPlanetDensityDistribution {
    pub const MIN: f64 = 0.3;
    pub const MAX: f64 = 1.1;
}

impl Distribution<f64> for GiantPlanetDensityDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let x = rng
            .sample(Normal::<f64>::new(0.5, 0.25).unwrap())
            .clamp(0., 1.);
        x * (Self::MAX - Self::MIN) + Self::MIN
    }
}

#[derive(Clone, Copy)]
pub struct MoonMassDistribution;

impl MoonMassDistribution {
    pub const MIN: f64 = 1e10;
    pub const MAX: f64 = 1e20;
    pub const PDF_MAX: f64 = 1.;
}

impl Distribution<f64> for MoonMassDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let x_distr = Uniform::new(0., 1.);
        let y_distr = Uniform::new(0., Self::PDF_MAX);

        loop {
            let x = rng.sample(x_distr);
            let y = rng.sample(y_distr);
            if y < moon_mass_pdf()(x) {
                return x * (Self::MAX - Self::MIN) + Self::MIN;
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct MoonDensityDistribution;

impl MoonDensityDistribution {
    pub const MIN: f64 = 1.5;
    pub const MAX: f64 = 3.5;
    pub const PDF_MAX: f64 = 1.;
}

impl Distribution<f64> for MoonDensityDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let x = rng
            .sample(Normal::<f64>::new(0.5, 0.25).unwrap())
            .clamp(0., 1.);
        x * (Self::MAX - Self::MIN) + Self::MIN
    }
}

#[derive(Clone, Copy)]
pub struct MoonSmiDistIncreCoeffDistribution;

impl MoonSmiDistIncreCoeffDistribution {
    pub const MIN: f64 = 1.;
    pub const MAX: f64 = 4.;
    pub const PDF_MAX: f64 = 1.;
}

impl Distribution<f64> for MoonSmiDistIncreCoeffDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let x_distr = Uniform::new(0., 1.);
        let y_distr = Uniform::new(0., Self::PDF_MAX);

        loop {
            let x = rng.sample(x_distr);
            let y = rng.sample(y_distr);
            if y < moon_smi_dist_incre_coeff_pdf()(x) {
                return x * (Self::MAX - Self::MIN) + Self::MIN;
            }
        }
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

    #[test]
    fn test_moon_mass() {
        let mut records = vec![0; 23];
        for _ in 0..10 {
            let mut rng = rand::thread_rng();
            let mass = MoonMassDistribution.sample(&mut rng);
            records[mass.log10() as usize] += 1;
        }
        records.iter().enumerate().for_each(|(i, &x)| {
            println!("{}: {}", i, x);
        });
    }

    #[test]
    fn test_moon_density() {
        let mut records = vec![0; 100];
        for _ in 0..10 {
            let mut rng = rand::thread_rng();
            let mass = MoonDensityDistribution.sample(&mut rng);
            records[(mass * 10.) as usize] += 1;
        }
        records.iter().enumerate().for_each(|(i, &x)| {
            println!("{}: {}", i, x);
        });
    }

    #[test]
    fn test_gas_giant_density() {
        let mut records = vec![0; 12];
        for _ in 0..10 {
            let mut rng = rand::thread_rng();
            let mass = GiantPlanetDensityDistribution.sample(&mut rng);
            records[(mass * 10.) as usize] += 1;
        }
        records.iter().enumerate().for_each(|(i, &x)| {
            println!("{}: {}", i, x);
        });
    }

    #[test]
    fn test_moon_smi_dist_incre_coeff() {
        let mut records = vec![0; 20];
        for _ in 0..10 {
            let mut rng = rand::thread_rng();
            let mass = MoonSmiDistIncreCoeffDistribution.sample(&mut rng);
            records[(mass * 10.) as usize] += 1;
        }
        records.iter().enumerate().for_each(|(i, &x)| {
            println!("{}: {}", i, x);
        });
    }
}
