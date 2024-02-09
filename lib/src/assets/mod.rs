use bevy::{
    app::{App, Plugin},
    asset::{Asset, AssetServer, Assets, Handle},
    ecs::{
        system::Resource,
        world::{FromWorld, World},
    },
    render::{
        color::Color,
        mesh::{shape::Circle, Mesh},
    },
    sprite::ColorMaterial,
    text::Font,
    utils::HashMap,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    consts,
    input::{camera::CameraController, KeyMapping},
    sim::components::CelestialBodyId,
    utils,
};

use self::settings::{StarProperties, UnitsInfo};

pub mod settings;

pub struct CosmosAssetsPlugin;

impl Plugin for CosmosAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalConfig>()
            .init_resource::<StarProperties>()
            .init_resource::<UnitsInfo>()
            .init_resource::<MeshAssets>()
            .init_resource::<MaterialAssets>()
            .init_resource::<FontAssets>();
    }
}

pub trait CelestialBodyAssets<T: Asset> {
    #[inline]
    fn get(&self, id: CelestialBodyId) -> Option<Handle<T>> {
        self.assets().get(&id).map(|h| h.clone_weak())
    }

    #[inline]
    fn insert(&mut self, id: CelestialBodyId, handle: Handle<T>) {
        self.assets_mut().insert(id, handle);
    }

    fn assets(&self) -> &HashMap<CelestialBodyId, Handle<T>>;
    fn assets_mut(&mut self) -> &mut HashMap<CelestialBodyId, Handle<T>>;
}

macro_rules! impl_asset {
    ($ty:ty, $asset_ty:ty) => {
        impl CelestialBodyAssets<$asset_ty> for $ty {
            #[inline]
            fn assets(&self) -> &HashMap<CelestialBodyId, Handle<$asset_ty>> {
                &self.assets
            }

            #[inline]
            fn assets_mut(&mut self) -> &mut HashMap<CelestialBodyId, Handle<$asset_ty>> {
                &mut self.assets
            }
        }
    };
}

#[derive(Resource)]
pub struct FontAssets {
    pub pixel_10px: Handle<Font>,
}

impl FromWorld for FontAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        FontAssets {
            pixel_10px: asset_server.load("fonts/ark-pixel-10px-monospaced-zh_cn.ttf"),
        }
    }
}

#[derive(Resource)]
pub struct MeshAssets {
    assets: HashMap<CelestialBodyId, Handle<Mesh>>,
    config: MeshConfig,
}
impl_asset!(MeshAssets, Mesh);

impl FromWorld for MeshAssets {
    fn from_world(world: &mut World) -> Self {
        let config = world.resource::<GlobalConfig>();
        Self {
            assets: HashMap::default(),
            config: config.mesh_config.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MeshConfig {
    pub base_radius: f64,
    pub segments: usize,
}

impl MeshAssets {
    pub fn generate(
        &mut self,
        assets: &mut Assets<Mesh>,
        id: CelestialBodyId,
        radius_px: f64,
    ) -> Handle<Mesh> {
        let handle = assets.add(
            Circle {
                radius: radius_px as f32,
                vertices: self.config.segments
                    * (radius_px / self.config.base_radius).ceil() as usize,
            }
            .into(),
        );
        self.insert(id, handle.clone());
        handle
    }
}

#[derive(Resource, Default)]
pub struct MaterialAssets {
    assets: HashMap<CelestialBodyId, Handle<ColorMaterial>>,
}
impl_asset!(MaterialAssets, ColorMaterial);

impl MaterialAssets {
    pub fn generate(
        &mut self,
        assets: &mut Assets<ColorMaterial>,
        id: CelestialBodyId,
        rng: &mut impl Rng,
    ) -> (Handle<ColorMaterial>, Color) {
        let color = Color::Rgba {
            red: rng.gen_range(0f32..1.),
            green: rng.gen_range(0f32..1.),
            blue: rng.gen_range(0f32..1.),
            alpha: 1.,
        };
        let handle = assets.add(ColorMaterial {
            color,
            ..Default::default()
        });
        self.insert(id, handle.clone());
        (handle, color)
    }
}

#[derive(Resource, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub key_mapping: KeyMapping,
    pub camera_controller: CameraController,
    pub mesh_config: MeshConfig,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        utils::deser(consts::GLOBAL_CONFIG).unwrap()
    }
}
