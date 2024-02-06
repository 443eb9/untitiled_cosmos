use bevy::app::{App, FixedUpdate, Plugin};

use self::resources::{Galaxy, TimeScale};

pub mod bundles;
pub mod components;
pub mod resources;
pub mod systems;

pub struct CosmosSimPlugin;

impl Plugin for CosmosSimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (systems::universal_gravitation, systems::transform_syncer),
        );

        app.init_resource::<TimeScale>().init_resource::<Galaxy>();

        #[cfg(feature = "debug")]
        {
            app.register_type::<components::CelestialBodyId>()
                .register_type::<components::CelestialBodySystemId>()
                .register_type::<components::CelestialBodyName>()
                .register_type::<components::StarClass>();

            app.register_type::<Galaxy>()
                .register_type::<resources::StarSystem>()
                .register_type::<resources::CelestialBody>();
        }
    }
}
