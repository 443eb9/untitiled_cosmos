use bevy::{
    app::{App, Plugin, Update},
    ecs::system::Resource,
    input::keyboard::KeyCode,
};
use serde::{Deserialize, Serialize};

use crate::assets::GlobalConfig;

use self::camera::CameraTarget;

pub mod camera;

pub struct CosmosInputPlugin;

impl Plugin for CosmosInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera::camera_control);

        let config = app.world.resource::<GlobalConfig>();
        app.insert_resource(config.key_mapping.clone());

        app.init_resource::<CameraTarget>();

        #[cfg(feature = "debug")]
        {
            use camera::*;

            app.register_type::<CameraController>()
                .register_type::<CameraTarget>();
        }
    }
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct KeyMapping {
    pub camera_up: KeyCode,
    pub camera_down: KeyCode,
    pub camera_left: KeyCode,
    pub camera_right: KeyCode,
}
