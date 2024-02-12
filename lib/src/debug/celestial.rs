use bevy::{
    asset::Assets,
    ecs::{
        reflect::ReflectResource,
        system::{Commands, Res, ResMut, Resource},
    },
    math::DVec2,
    reflect::Reflect,
    render::mesh::Mesh,
    sprite::ColorMaterial,
};

use crate::{
    assets::{
        settings::{ConstellationNames, StarProperties},
        MaterialAssets, MeshAssets, SubstanceAssets,
    },
    gen::{GalaxyGenerator, GalaxyGeneratorConfig},
    sim::{
        bundles::CelestialBodyBundle, components::CelestialBodyId, resources::{CelestialBody, Galaxy}
    },
};

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct BodyGenerator {
    pub body: CelestialBody,
}

impl Default for BodyGenerator {
    fn default() -> Self {
        Self {
            body: CelestialBody::new(DVec2::ZERO, 0., 0., DVec2::ZERO),
        }
    }
}

pub fn generate_galaxy(
    mut commands: Commands,
    properties: Res<StarProperties>,
    constellation_names: Res<ConstellationNames>,
    substance_assets: Res<SubstanceAssets>,
    mut mesh_assets: ResMut<MeshAssets>,
    mut material_assets: ResMut<MaterialAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let config = GalaxyGeneratorConfig::new_debug();
    let mut generator = GalaxyGenerator::new(
        config,
        &constellation_names,
        &properties,
        &substance_assets,
        &mut mesh_assets,
        &mut material_assets,
        &mut meshes,
        &mut materials,
    );
    generator.generate();
    let mut galaxy = Galaxy::default();
    let mut bundles = Vec::new();
    generator.transfer_result(&mut galaxy, &mut bundles);
    commands.insert_resource(galaxy);
    bundles.into_iter().for_each(|(cb, mb)| {
        let mut commands = match cb {
            CelestialBodyBundle::Star(b) => commands.spawn(b),
            CelestialBodyBundle::Planet {
                planet,
                crust,
                atmo,
            } => {
                let mut entity = commands.spawn(planet.clone());
                if let Some(crust) = crust {
                    entity.insert(crust);
                }
                if let Some(atmo) = atmo {
                    entity.insert(atmo);
                }
                entity
            }
            CelestialBodyBundle::Moon { moon, crust, atmo } => {
                let mut entity = commands.spawn((moon, crust));
                if let Some(atmo) = atmo {
                    entity.insert(atmo);
                }
                entity
            }
        };
        commands.insert(mb);
    });
}

// pub fn body_removal_test(mut galaxy: ResMut<Galaxy>) {
//     galaxy.remove_body(CelestialBodyId(2));
//     println!("removed 2");
//     galaxy.remove_body(CelestialBodyId(4));
//     println!("removed 4");
//     galaxy.remove_body(CelestialBodyId(0));
//     println!("removed 0");
// }

#[cfg(test)]
mod test {
    use rand_distr::Distribution;

    use crate::gen::distr::StarMassDistribution;

    #[test]
    fn test_mass_distr() {
        let mut records = vec![0; 120];
        for _ in 0..64 {
            let mut rng = rand::thread_rng();
            let mass = StarMassDistribution.sample(&mut rng);
            records[mass as usize] += 1;
        }
        println!("{:?}", records);
    }
}
