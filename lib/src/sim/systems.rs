use bevy::{
    ecs::system::{Query, Res, ResMut},
    transform::components::Transform,
};

use super::{
    components::CelestialBodySystemId,
    resources::{Galaxy, TimeScale},
};

pub(super) fn universal_gravitation(mut galaxy: ResMut<Galaxy>, time_scale: Res<TimeScale>) {
    galaxy.calc_acc();
    galaxy.update_pos(time_scale.0);
}

pub(super) fn transform_syncer(
    galaxy: Res<Galaxy>,
    mut bodies_query: Query<(&CelestialBodySystemId, &mut Transform)>,
) {
    bodies_query.par_iter_mut().for_each(|(id, mut transform)| {
        if let Some(body) = galaxy.get_body(*id) {
            transform.translation = body.pos().as_vec2().extend(0.);
        }
    });
}
