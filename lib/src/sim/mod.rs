use bevy::app::{App, FixedUpdate, Plugin, Update};

use self::resources::{OrbitPredictor, SimulationTimeScale};

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

        app.add_systems(Update, systems::orbit_drawer);

        app.init_resource::<OrbitPredictor>()
            .init_resource::<SimulationTimeScale>();

        #[cfg(feature = "debug")]
        {
            use components::*;
            use resources::*;

            app.register_type::<CelestialBodyId>()
                .register_type::<CelestialBodySystemId>()
                .register_type::<CelestialBodyName>()
                .register_type::<SpectralType>();

            app.register_type::<Galaxy>()
                .register_type::<OrbitPredictor>()
                .register_type::<StarSystem>()
                .register_type::<CelestialBody>()
                .register_type::<SimulationTimeScale>();
        }
    }
}
