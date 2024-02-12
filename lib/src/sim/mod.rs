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
                .register_type::<CelestialBodyName>()
                .register_type::<CelestialBodyColor>()
                .register_type::<CelestialBodyCrust>()
                .register_type::<CelestialBodySubstanceProps>()
                .register_type::<CelestialBodyAtmosphere>();

            app.register_type::<SpectralType>()
                .register_type::<StarClass>()
                .register_type::<CelestialBodyEffectiveTemp>()
                .register_type::<StarLuminosity>();

            app.register_type::<PlanetType>();

            app.register_type::<Galaxy>()
                .register_type::<OrbitPredictor>()
                .register_type::<CelestialBody>()
                .register_type::<SimulationTimeScale>();
        }
    }
}
