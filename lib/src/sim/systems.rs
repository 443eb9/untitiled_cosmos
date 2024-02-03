use bevy::ecs::system::{Res, ResMut};

use super::resources::{TimeScale, Universe};

pub(super) fn universal_gravitation(mut universe: ResMut<Universe>, time_scale: Res<TimeScale>) {
    universe.calc_acc();
    universe.update_pos(time_scale.0);
}
