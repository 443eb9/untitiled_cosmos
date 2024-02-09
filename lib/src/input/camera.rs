use bevy::{
    ecs::{
        component::Component,
        event::EventReader,
        system::{Query, Res, ResMut, Resource},
    },
    input::{keyboard::KeyCode, mouse::MouseWheel, Input},
    math::Vec3,
    render::camera::OrthographicProjection,
    time::Time,
    transform::components::Transform,
};
use serde::{Deserialize, Serialize};

use super::KeyMapping;

#[cfg(feature = "debug")]
use bevy::{ecs::reflect::ReflectResource, reflect::Reflect};

#[derive(Component, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "debug", derive(Reflect))]
pub struct CameraController {
    pub move_speed: f32,
    pub move_lerp_rate: f32,
    pub zoom_speed: f32,
    pub zoom_lerp_rate: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
}

#[derive(Resource)]
#[cfg_attr(feature = "debug", derive(Reflect))]
#[cfg_attr(feature = "debug", reflect(Resource))]
pub struct CameraTarget {
    pub position: Vec3,
    pub zoom: f32,
}

impl Default for CameraTarget {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            zoom: 300.,
        }
    }
}

pub fn camera_control(
    mut camera_query: Query<(
        &mut Transform,
        &mut OrthographicProjection,
        &CameraController,
    )>,
    key_mapping: Res<KeyMapping>,
    input_keyboard: Res<Input<KeyCode>>,
    mut input_mouse: EventReader<MouseWheel>,
    time: Res<Time>,
    mut target: ResMut<CameraTarget>,
) {
    let Ok((mut transform, mut projection, controller)) = camera_query.get_single_mut() else {
        return;
    };

    let mut translation = Vec3::ZERO;
    if input_keyboard.pressed(key_mapping.camera_up) {
        translation += Vec3::Y;
    }
    if input_keyboard.pressed(key_mapping.camera_down) {
        translation -= Vec3::Y;
    }
    if input_keyboard.pressed(key_mapping.camera_left) {
        translation -= Vec3::X;
    }
    if input_keyboard.pressed(key_mapping.camera_right) {
        translation += Vec3::X;
    }
    target.position +=
        controller.move_speed * time.delta_seconds() * translation * projection.scale;

    for event in input_mouse.read() {
        target.zoom -= controller.zoom_speed * event.y * target.zoom.sqrt();
    }
    target.zoom = target.zoom.clamp(controller.min_zoom, controller.max_zoom);

    if transform.translation.distance_squared(target.position) > 0.001 {
        transform.translation = transform.translation.lerp(
            target.position,
            controller.move_lerp_rate * time.delta_seconds(),
        );
    }

    if (projection.scale - target.zoom).abs() > 0.001 {
        projection.scale = projection.scale
            + (target.zoom - projection.scale) * controller.zoom_lerp_rate * time.delta_seconds();
    }
}
