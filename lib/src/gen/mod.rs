use bevy::{
    asset::Assets,
    log::info,
    math::DVec2,
    render::mesh::Mesh,
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::{rngs::StdRng, Rng};
use rand_distr::Uniform;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    assets::{
        settings::{StarProperties, UnitsInfo},
        MaterialAssets, MeshAssets,
    },
    consts,
    sim::{
        bundles::{CelestialBodyBundle, StarBundle},
        components::{CelestialBodyId, CelestialBodyName, CelestialBodySystemId, SpectralType},
        resources::{CelestialBody, Galaxy, OrbitPredictor},
    },
};

use self::distr::{StarMassDistribution, StarPosDistribution};

pub mod distr;
pub mod physics;

pub struct GalaxyGeneratorConfig {
    pub seed: u64,
    pub galaxy_name: String,
    pub galaxy_radius: f64,
    pub num_stars: usize,
    pub base_num_planets: usize,
    pub planets_mass_related_coeff: f32,
    pub coeff_num_planets: Uniform<f32>,
    pub base_num_moons: usize,
    pub moons_mass_related_coeff: f32,
    pub coeff_num_moons: Uniform<f32>,
}

impl GalaxyGeneratorConfig {
    pub fn new_debug() -> Self {
        Self {
            seed: 0,
            galaxy_name: "Milky Way".to_string(),
            galaxy_radius: 1. * consts::LY,
            num_stars: 64,
            base_num_planets: 6,
            planets_mass_related_coeff: 0.5,
            coeff_num_planets: Uniform::new(0.5, 1.5),
            base_num_moons: 3,
            moons_mass_related_coeff: 0.5,
            coeff_num_moons: Uniform::new(0.5, 1.5),
        }
    }
}

pub struct GalaxyGenerator<'a> {
    cfg: GalaxyGeneratorConfig,
    rng: StdRng,
    star_props: &'a StarProperties,
    units: &'a UnitsInfo,
    mesh_asstes: &'a mut MeshAssets,
    material_assets: &'a mut MaterialAssets,
    meshes: &'a mut Assets<Mesh>,
    materials: &'a mut Assets<ColorMaterial>,
    galaxy: Galaxy,
    bundles: Vec<(CelestialBodyBundle, MaterialMesh2dBundle<ColorMaterial>)>,
}

impl<'a> GalaxyGenerator<'a> {
    pub fn new(
        config: GalaxyGeneratorConfig,
        star_props: &'a StarProperties,
        units: &'a UnitsInfo,
        mesh_asstes: &'a mut MeshAssets,
        material_assets: &'a mut MaterialAssets,
        meshes: &'a mut Assets<Mesh>,
        materials: &'a mut Assets<ColorMaterial>,
    ) -> Self {
        Self {
            rng: rand::SeedableRng::seed_from_u64(config.seed),
            cfg: config,
            star_props,
            units,
            mesh_asstes,
            material_assets,
            meshes,
            materials,
            galaxy: Galaxy::default(),
            bundles: Vec::new(),
        }
    }

    pub fn transfer_result(
        &mut self,
        galaxy: &mut Galaxy,
        bundles: &mut Vec<(CelestialBodyBundle, MaterialMesh2dBundle<ColorMaterial>)>,
    ) {
        std::mem::swap(&mut self.galaxy, galaxy);
        std::mem::swap(&mut self.bundles, bundles);
    }

    pub fn generate(&mut self) {
        info!("Galaxy generation started");
        for _ in 0..self.cfg.num_stars {
            self.gen_star();

            let num_planets = (self.cfg.base_num_planets as f32
                * self.rng.sample(self.cfg.coeff_num_planets))
                as usize;
            for _ in 0..num_planets {}
        }
    }

    fn gen_star(&mut self) {
        let spd = StarPosDistribution::new(self.cfg.galaxy_radius);
        let smd = StarMassDistribution;

        loop {
            info!(
                "Generating stars {}/{}",
                self.galaxy.system_count(),
                self.cfg.num_stars
            );
            let mass = self.rng.sample(smd);
            let pos = {
                loop {
                    let pos = self.rng.sample(spd);
                    let num_confl = self
                        .galaxy
                        .systems()
                        .par_iter()
                        .filter(|system| {
                            let other = self.galaxy.get_body(system.bodies[0]).unwrap();
                            let f =
                                consts::G * other.mass() * mass / other.pos().distance_squared(pos);
                            let a1 = f / mass;
                            let a2 = f / other.mass();

                            a1 > consts::ACC_THRESHOLD || a2 > consts::ACC_THRESHOLD
                        })
                        .collect::<Vec<_>>()
                        .len();
                    if num_confl == 0 {
                        break pos;
                    }
                }
            };

            let (bound_floor, bound_ceil) = self.star_props.find_bound(mass, |info| info.mass);
            let percent = (mass - bound_floor.mass) / (bound_ceil.mass - bound_floor.mass);
            let radius = bound_floor.radius + (bound_ceil.radius - bound_floor.radius) * percent;

            let star = CelestialBody::new(
                pos,
                radius * consts::SUN_RADIUS * consts::STAR_RADIUS_SCALE,
                mass * consts::SUN_MASS * consts::STAR_MASS_SCALE,
                DVec2::ZERO,
            );

            self.galaxy.create_system();

            let (id, system_id) = self.galaxy.add_body(self.galaxy.system_count() - 1, star);

            let mesh = Mesh2dHandle(self.mesh_asstes.generate(self.meshes, id, star.radius()));
            let (material, color) =
                self.material_assets
                    .generate(self.materials, id, &mut self.rng);

            self.galaxy.set_color(id, color);

            let bundle = (
                CelestialBodyBundle::Star(StarBundle {
                    id,
                    system_id,
                    name: CelestialBodyName("".to_string()),
                    class: bound_floor.class,
                }),
                MaterialMesh2dBundle {
                    mesh,
                    material,
                    ..Default::default()
                },
            );

            self.bundles.push(bundle);

            break;
        }
    }
}
