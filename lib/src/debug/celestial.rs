use bevy::{
    asset::Assets,
    ecs::{
        reflect::ReflectResource,
        system::{Commands, Res, ResMut, Resource},
    },
    input::{keyboard::KeyCode, Input},
    math::DVec2,
    reflect::Reflect,
    render::mesh::Mesh,
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    assets::{
        settings::{StarProperties, UnitsInfo},
        MaterialAssets, MeshAssets,
    },
    consts,
    gen::{GalaxyGenerator, GalaxyGeneratorConfig},
    math::aabbs::Aabb2d,
    sim::{
        bundles::{CelestialBodyBundle, StarBundle},
        components::{CelestialBodyName, SpectralType, StarClass},
        resources::{CelestialBody, Galaxy},
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

pub fn spawn_body(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    body_gen: Res<BodyGenerator>,
    mut galaxy: ResMut<Galaxy>,
    mut meshes: ResMut<MeshAssets>,
    mut materials: ResMut<MaterialAssets>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<ColorMaterial>>,
) {
    if input.just_pressed(KeyCode::F1) {
        let (id, sys_id) = galaxy.add_body(0, body_gen.body.clone());
        let (material, color) =
            materials.generate(&mut material_assets, id, &mut rand::thread_rng());
        galaxy.set_color(id, color);
        commands.spawn((
            StarBundle {
                id,
                system_id: sys_id,
                name: CelestialBodyName("".to_string()),
                class: StarClass {
                    ty: SpectralType::O,
                    sub_ty: 0,
                },
            },
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.generate(&mut mesh_assets, id, body_gen.body.radius())),
                material,
                ..Default::default()
            },
        ));
    }
}

pub fn generate_galaxy(
    mut commands: Commands,
    units: Res<UnitsInfo>,
    properties: Res<StarProperties>,
    mut mesh_assets: ResMut<MeshAssets>,
    mut material_assets: ResMut<MaterialAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let config = GalaxyGeneratorConfig::new_debug();
    let mut generator = GalaxyGenerator::new(
        config,
        &properties,
        &units,
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
            CelestialBodyBundle::Planet(b) => commands.spawn(b),
        };
        commands.insert(mb);
    });
}

#[cfg(test)]
mod test {
    use rand_distr::Distribution;

    use crate::gen::distr::StarMassDistribution;

    use super::*;

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
