use bevy::app::{App, FixedUpdate, Plugin};

use self::resources::{TimeScale, Universe};

pub mod bundles;
pub mod components;
pub mod resources;
pub mod systems;

pub struct CosmosSimPlugin;

impl Plugin for CosmosSimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, systems::universal_gravitation);

        app.init_resource::<TimeScale>().init_resource::<Universe>();
    }
}
