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
    assets::{MaterialAssets, MeshAssets},
    sim::{
        bundles::StarBundle,
        components::{CelestialBodyName, StarClass},
        resources::{CelestialBody, Galaxy, StarSystem},
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
    if input.just_pressed(KeyCode::Space) {
        if galaxy.system_count() == 0 {
            galaxy.add_system(StarSystem::new(0, DVec2::ZERO, 100.));
        }

        let (id, sys_id) = galaxy.add_body(0, body_gen.body.clone());
        commands.spawn((
            StarBundle {
                id,
                system_id: sys_id,
                name: CelestialBodyName("".to_string()),
                star_class: StarClass::O,
            },
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.generate(&mut mesh_assets, id, body_gen.body.radius())),
                material: materials.generate(&mut material_assets, id, &mut rand::thread_rng()),
                ..Default::default()
            },
        ));
    }
}
