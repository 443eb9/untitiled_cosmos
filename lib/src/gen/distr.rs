use bevy::{math::DVec2, utils::HashMap};
use rand::Rng;
use rand_distr::{Distribution, Normal, Uniform};

use crate::{consts, sci::chemistry::SubstanceContent};

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
        2. * (-(150. * x).powi(5) + 5.).max(0.)
            + ((300. * x - 20.).tanh() - (3. * x - 1.).tanh()) * 0.2
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
pub fn rocky_body_atmo_density_pdf() -> impl Fn(f64) -> f64 {
    move |x| (-x * x * x * x + 1.).powi(2)
}

#[inline]
pub fn rocky_body_atmo_density_to_opacity(mut density: f64) -> f64 {
    density *= 1000.;
    density * density / 4.
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
    pub const PDF_MAX: f64 = 9.952;
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
pub struct RockyBodyCrustDensityDistribution;

impl RockyBodyCrustDensityDistribution {
    pub const MIN: f64 = 2.5;
    pub const MAX: f64 = 5.5;
}

impl Distribution<f64> for RockyBodyCrustDensityDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let x = rng
            .sample(Normal::<f64>::new(0.5, 0.25).unwrap())
            .clamp(0., 1.);
        x * (Self::MAX - Self::MIN) + Self::MIN
    }
}

#[derive(Clone, Copy)]
pub struct RockyBodyAtmoDensityDistribution;

impl RockyBodyAtmoDensityDistribution {
    pub const MIN: f64 = 0.2e-3;
    pub const MAX: f64 = 2.5e-3;
    pub const PDF_MAX: f64 = 0.3277;
}

impl Distribution<f64> for RockyBodyAtmoDensityDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let x_distr = Uniform::new(0., 1.);
        let y_distr = Uniform::new(0., Self::PDF_MAX);

        loop {
            let x = rng.sample(x_distr);
            let y = rng.sample(y_distr);
            if y < rocky_body_atmo_density_pdf()(x) {
                return x * (Self::MAX - Self::MIN) + Self::MIN;
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct GiantPlanetDensityDistribution;

impl GiantPlanetDensityDistribution {
    pub const MIN: f64 = 0.3;
    pub const MAX: f64 = 1.8;
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
    pub const MIN: f64 = 1.2;
    pub const MAX: f64 = 2.;
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

// NOTICE: These distributions are not normalized!!!!

macro_rules! impl_composition_distr {
    ($distr:ident, $list:ident) => {
        impl Distribution<SubstanceContent> for $distr {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SubstanceContent {
                let mut composition = HashMap::with_capacity(consts::$list.len());
                consts::$list.into_iter().for_each(|(comp, content)| {
                    composition.insert(comp, rng.gen_range(content));
                });

                SubstanceContent::new(composition)
            }
        }
    };
}

#[derive(Clone, Copy)]
pub struct StarCompositionDistribution;
impl_composition_distr!(StarCompositionDistribution, STAR_COMPOSITION);

#[derive(Clone, Copy)]
pub struct GasGiantCompositionDistribution;
impl_composition_distr!(GasGiantCompositionDistribution, GAS_GIANT_ATMO);

#[derive(Clone, Copy)]
pub struct IceGiantCompositionDistribution;
impl_composition_distr!(IceGiantCompositionDistribution, ICE_GIANT_ATMO);

#[derive(Clone, Copy)]
pub struct RockyCrustCompositionDistribution;
impl_composition_distr!(RockyCrustCompositionDistribution, ROCKY_PLANET_CRUST);

#[derive(Clone, Copy)]
pub struct RockyAtmosphereCompositionDistribution;
impl_composition_distr!(RockyAtmosphereCompositionDistribution, ROCKY_PLANET_ATMO);

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
    fn test_planet_mass() {
        let mut records = vec![0; 268];
        for _ in 0..64 {
            let mut rng = rand::thread_rng();
            let mass = PlanetMassDistribution.sample(&mut rng);
            records[mass as usize] += 1;
        }
        println!("{:?}", records);
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

    #[test]
    fn test_comp_distr() {
        let mut rng = rand::thread_rng();
        let comp = StarCompositionDistribution.sample(&mut rng);
        println!("star: {:?}", comp.normalized());

        let comp = GasGiantCompositionDistribution.sample(&mut rng);
        println!("gas giant: {:?}", comp.normalized());

        let comp = IceGiantCompositionDistribution.sample(&mut rng);
        println!("ice giant: {:?}", comp.normalized());

        let comp = RockyCrustCompositionDistribution.sample(&mut rng);
        println!("rocky crust: {:?}", comp.normalized());

        let comp = RockyAtmosphereCompositionDistribution.sample(&mut rng);
        println!("rocky atmo: {:?}", comp.normalized());

        let density = RockyBodyAtmoDensityDistribution.sample(&mut rng);
        println!("rocky atmo density: {}", density);
    }
}
