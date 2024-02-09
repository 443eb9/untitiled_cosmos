use std::f64::consts::PI;

use bevy::{
    asset::Assets,
    log::info,
    math::DVec2,
    render::mesh::Mesh,
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rand_distr::Uniform;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    assets::{
        settings::{StarProperties, UnitsInfo},
        MaterialAssets, MeshAssets,
    },
    consts, math,
    sim::{
        bundles::{CelestialBodyBundle, PlanetBundle, StarBundle},
        components::{CelestialBodyId, CelestialBodyName, CelestialBodySystemId, SpectralType},
        resources::{CelestialBody, Galaxy, OrbitPredictor},
    },
};

use self::distr::{
    GasGiantDensityDistribution, PlanetMassDistribution, PlanetSmiDistDistribution,
    StarMassDistribution, StarPosDistribution,
};

pub mod distr;
pub mod physics;

pub struct PlanetGenerationConfig {
    pub num_coeff: Uniform<f32>,
    /// Minimum linear speed of a planet at farthest point from the star.
    pub min_revl_spd: f64,
    pub sma_smi_ratio: Uniform<f64>,
}

pub struct GalaxyGeneratorConfig {
    pub seed: u64,
    pub galaxy_name: String,
    pub galaxy_radius: f64,
    pub num_stars: usize,
    pub pln_cfg: PlanetGenerationConfig,
    pub coeff_num_moons: Uniform<f32>,
}

impl GalaxyGeneratorConfig {
    pub fn new_debug() -> Self {
        Self {
            seed: 0,
            galaxy_name: "Milky Way".to_string(),
            galaxy_radius: 1000.,
            num_stars: 1,
            pln_cfg: PlanetGenerationConfig {
                num_coeff: Uniform::new(0.6, 1.3),
                min_revl_spd: 5.,
                sma_smi_ratio: Uniform::new(1., 1.5),
            },
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
    smi_dist: Vec<f64>,
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
            rng: SeedableRng::seed_from_u64(config.seed),
            cfg: config,
            star_props,
            units,
            mesh_asstes,
            material_assets,
            meshes,
            materials,
            galaxy: Galaxy::default(),
            bundles: Vec::new(),
            smi_dist: Vec::new(),
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
        let start = std::time::SystemTime::now();
        info!("Galaxy generation started");
        for sys_id in 0..self.cfg.num_stars {
            info!("Generating stars {}/{}", sys_id, self.cfg.num_stars);

            let star_id = self.gen_star();
            let (CelestialBodyBundle::Star(star_bundle), _) = self.bundles.get(star_id.0).unwrap()
            else {
                unreachable!()
            };
            let star_body = self.galaxy.get_body(star_id).unwrap();
            let num_planets = distr::max_num_planets(
                star_bundle.class.to_index(),
                self.cfg.pln_cfg.num_coeff,
                &mut self.rng,
            );
            let system_edge =
                physics::min_spd_to_dist(self.cfg.pln_cfg.min_revl_spd, star_body.mass());

            for plnt_id in 0..num_planets {
                info!(
                    "Generating planets {}/{}[in system {}]",
                    plnt_id, num_planets, sys_id
                );
                self.gen_planet(sys_id, system_edge);
            }
        }
        let elapsed = start.elapsed().unwrap();

        info!("Galaxy generation finished in {}ms", elapsed.as_millis());
    }

    fn gen_star(&mut self) -> CelestialBodyId {
        let spd = StarPosDistribution::new(self.cfg.galaxy_radius);
        let smd = StarMassDistribution;

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
                        let f = physics::force_between(
                            &CelestialBody::new(pos, 0., mass, DVec2::ZERO),
                            other,
                        );
                        let a1 = f / mass;
                        let a2 = f / other.mass();

                        a1 > consts::STAR_ACC_THRESHOLD || a2 > consts::STAR_ACC_THRESHOLD
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

        let (id, systemic_id) = self.galaxy.add_body(self.galaxy.system_count() - 1, star);

        let mesh = Mesh2dHandle(self.mesh_asstes.generate(self.meshes, id, star.radius()));
        let (material, color) = self
            .material_assets
            .generate(self.materials, id, &mut self.rng);

        self.galaxy.set_color(id, color);

        let bundle = (
            CelestialBodyBundle::Star(StarBundle {
                id,
                systemic_id,
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
        self.smi_dist.push(0.);

        id
    }

    pub fn gen_planet(&mut self, system_id: usize, system_edge: f64) {
        let system = self.galaxy.get_system(system_id).unwrap();
        let star_id = system.bodies[0];
        let star = self.galaxy.get_body(star_id).unwrap();

        let mass = self.rng.sample(PlanetMassDistribution)
            * consts::EARTH_MASS
            * consts::PLANET_MASS_SCALE;

        if mass / consts::EARTH_MASS / consts::PLANET_MASS_SCALE < consts::GAS_GIANT_MASS_THRESHOLD
        {
            println!("rocky planet");
        } else {
            println!("gas giant");
        }
        let density = self.rng.sample(GasGiantDensityDistribution);
        let radius = math::mass_to_radius(mass, density)
            * consts::EARTH_RADIUS
            * consts::PLANET_RADIUS_SCALE;

        let (min_smi_dist, max_smi_dist) = {
            if system.bodies.len() == 1 {
                (
                    star.radius() * consts::BODY_TO_PARENT_DIST_COEFF + radius,
                    consts::BASE_BODY_DIST + star.radius() + radius,
                )
            } else {
                let last_planet_id = system.bodies.last().unwrap();
                let rhs = self.galaxy.get_body(*last_planet_id).unwrap();
                let d1 = physics::mass_acc_to_radius(mass, consts::PLANET_ACC_THRESHOLD);
                let d2 = physics::mass_acc_to_radius(rhs.mass(), consts::PLANET_ACC_THRESHOLD);
                let pos = self.smi_dist.last().unwrap();
                (d1.max(d2) + *pos, consts::BASE_BODY_DIST + *pos)
            }
        };

        if min_smi_dist > system_edge {
            info!(
                "Planet generation failed: Max planet distance exceeded. {} > {}",
                min_smi_dist, system_edge
            );
            return;
        }

        let smi_dist = self.rng.sample(PlanetSmiDistDistribution {
            min: min_smi_dist,
            max: max_smi_dist,
        });
        dbg!(smi_dist, system_edge, min_smi_dist);
        self.smi_dist.push(smi_dist);
        let sma_dist = smi_dist * self.rng.sample(self.cfg.pln_cfg.sma_smi_ratio);

        let init_vel = physics::vis_viva_get_smi_vel(star.mass() + mass, smi_dist, sma_dist);

        let body = CelestialBody::new(
            star.pos() + consts::DEFAULT_BODY_EXTEND_AXIS * smi_dist,
            radius,
            mass,
            init_vel * consts::DEFAULT_BODY_VEL_DIR,
        );

        let (id, systemic_id) = self.galaxy.add_body(system_id, body);

        let mesh = Mesh2dHandle(self.mesh_asstes.generate(self.meshes, id, radius));
        let (material, color) = self
            .material_assets
            .generate(self.materials, id, &mut self.rng);

        self.galaxy.set_color(id, color);

        self.bundles.push((
            CelestialBodyBundle::Planet(PlanetBundle {
                id,
                systemic_id,
                name: CelestialBodyName("".to_string()),
            }),
            MaterialMesh2dBundle {
                mesh,
                material,
                ..Default::default()
            },
        ));
    }
}
