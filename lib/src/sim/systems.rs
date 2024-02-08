use bevy::{
    ecs::system::{Query, Res, ResMut},
    gizmos::gizmos::Gizmos,
    transform::components::Transform,
};

use super::{
    components::CelestialBodyId,
    resources::{Galaxy, OrbitPredictor, SimulationTimeScale},
};

pub(super) fn universal_gravitation(
    mut galaxy: ResMut<Galaxy>,
    mut predictor: ResMut<OrbitPredictor>,
    time_scale: Res<SimulationTimeScale>,
) {
    for _ in 0..time_scale.0 {
        galaxy.step();
        predictor.step();
    }
}

pub(super) fn transform_syncer(
    galaxy: Res<Galaxy>,
    mut bodies_query: Query<(&CelestialBodyId, &mut Transform)>,
) {
    bodies_query.par_iter_mut().for_each(|(id, mut transform)| {
        if let Some(body) = galaxy.get_body(*id) {
            transform.translation = (body.pos().as_vec2() * 100.).extend(0.);
        }
    });
}

pub(super) fn orbit_drawer(predictor: Res<OrbitPredictor>, mut gizmos: Gizmos) {
    predictor.iter().for_each(|orbit| {
        let verts = orbit.vertices();
        if verts.is_empty() {
            return;
        }
        for i in 0..orbit.vertices().len() - 1 {
            gizmos.line_2d(
                verts[i].as_vec2() * 100.,
                verts[i + 1].as_vec2() * 100.,
                orbit.color,
            );
        }
    });
}
