use bevy::{
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, Res},
};

use crate::assets::GlobalConfig;

pub fn init(mut commands: Commands, config: Res<GlobalConfig>) {
    commands.spawn((Camera2dBundle::default(), config.camera_controller.clone()));
}
