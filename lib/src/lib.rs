use bevy::app::{App, Plugin};

pub mod consts;
pub mod core;
pub mod gen;
pub mod math;
pub mod nn;
pub mod sim;

pub struct CosmosPlugin;

impl Plugin for CosmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((core::CosmosInitializerPlugin, core::CosmosGamePlugin));
    }
}
