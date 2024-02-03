use bevy::app::{App, Plugin, Startup};

mod systems;

pub struct CosmosInitializerPlugin;

impl Plugin for CosmosInitializerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::init);
    }
}

pub struct CosmosGamePlugin;

impl Plugin for CosmosGamePlugin {
    fn build(&self, _app: &mut App) {}
}
