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
use serde::{Deserialize, Serialize};

use crate::{
    consts,
    input::{camera::CameraController, KeyMapping},
    math::HexRgbaColor,
    sci::chemistry::{Substance, SubstanceProperty},
    sim::components::{CelestialBodyId, PlanetType},
    utils,
};

use self::settings::{ConstellationNames, StarProperties, UnitsInfo};

pub mod settings;

pub struct CosmosAssetsPlugin;

impl Plugin for CosmosAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalConfig>()
            .init_resource::<ConstellationNames>()
            .init_resource::<StarProperties>()
            .init_resource::<SubstanceAssets>()
            .init_resource::<UnitsInfo>()
            .init_resource::<MeshAssets>()
            .init_resource::<MaterialAssets>()
            .init_resource::<FontAssets>()
            .init_resource::<BodyStylishAssets>();
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
        color: Color,
    ) -> Handle<ColorMaterial> {
        let handle = assets.add(ColorMaterial {
            color,
            ..Default::default()
        });
        self.insert(id, handle.clone());
        handle
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

#[derive(Resource)]
pub struct SubstanceAssets {
    assets: Vec<SubstanceProperty>,
}

impl Default for SubstanceAssets {
    fn default() -> Self {
        Self {
            assets: utils::deser(consts::SUBSTANCE_ASSETS).unwrap(),
        }
    }
}

impl SubstanceAssets {
    #[inline]
    pub fn get(&self, substance: Substance) -> &SubstanceProperty {
        &self.assets[substance as usize]
    }

    #[inline]
    pub fn assets(&self) -> &[SubstanceProperty] {
        &self.assets
    }

    #[inline]
    pub fn to_vec(self) -> Vec<SubstanceProperty> {
        self.assets
    }

    #[inline]
    pub fn generate_props_on(&self, atmo_density: f64) -> Self {
        Self {
            assets: self
                .assets
                .clone()
                .into_iter()
                .map(|prop| SubstanceProperty {
                    boiling_point: prop.get_boiling_point_at(atmo_density),
                    ..prop
                })
                .collect(),
        }
    }

    #[inline]
    pub fn generate_in_vaccum(&self) -> Self {
        Self {
            assets: self
                .assets
                .clone()
                .into_iter()
                .map(|prop| SubstanceProperty {
                    boiling_point: prop.melting_point,
                    ..prop
                })
                .collect(),
        }
    }
}

#[derive(Resource)]
pub struct BodyStylishAssets {
    assets: Vec<Vec<HexRgbaColor>>,
}

impl Default for BodyStylishAssets {
    fn default() -> Self {
        Self {
            assets: utils::deser(consts::BODY_STYLISH).unwrap(),
        }
    }
}

impl BodyStylishAssets {
    pub fn get(&self, ty: PlanetType) -> &Vec<HexRgbaColor> {
        &self.assets[ty as usize]
    }
}
