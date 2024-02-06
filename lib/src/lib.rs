use bevy::app::{App, Plugin};

pub mod assets;
pub mod consts;
pub mod core;
#[cfg(feature = "debug")]
pub mod debug;
pub mod gen;
pub mod input;
pub mod math;
pub mod sim;
pub mod utils;

pub struct CosmosPlugin;

impl Plugin for CosmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            assets::CosmosAssetsPlugin,
            core::CosmosInitializerPlugin,
            core::CosmosGamePlugin,
            sim::CosmosSimPlugin,
            input::CosmosInputPlugin,
            #[cfg(feature = "debug")]
            debug::CosmosDebugPlugin {
                inspector: true,
                body_spawn: true,
            },
        ));
    }
}
