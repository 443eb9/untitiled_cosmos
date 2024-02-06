use bevy::{
    app::{App, Plugin, Startup, Update},
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    ecs::{
        component::Component,
        system::{Commands, Query, Res},
    },
    hierarchy::BuildChildren,
    text::{Text, TextStyle},
    ui::node_bundles::{NodeBundle, TextBundle},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::assets::FontAssets;

use self::celestial::BodyGenerator;

mod celestial;

pub struct CosmosDebugPlugin {
    pub inspector: bool,
    pub body_spawn: bool,
}

impl Plugin for CosmosDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        if self.inspector {
            app.add_plugins(WorldInspectorPlugin::default());
        }

        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_ui);

        if self.body_spawn {
            app.add_systems(Update, celestial::spawn_body);
        }

        app.init_resource::<BodyGenerator>();

        app.register_type::<BodyGenerator>();
    }
}

#[derive(Component)]
pub struct FrameText;

pub fn setup_ui(mut commands: Commands, fonts: Res<FontAssets>) {
    commands.spawn(NodeBundle::default()).with_children(|root| {
        root.spawn((
            TextBundle {
                text: Text::from_section(
                    "",
                    TextStyle {
                        font: fonts.pixel_10px.clone_weak(),
                        font_size: 25.,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            },
            FrameText,
        ));
    });
}

pub fn update_ui(diag: Res<DiagnosticsStore>, mut frame_text: Query<&mut Text>) {
    let mut frame_text = frame_text.single_mut();
    if let (Some(rate), Some(time)) = (
        diag.get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|d| d.smoothed()),
        diag.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
            .and_then(|d| d.smoothed()),
    ) {
        frame_text.sections[0].value = format!("FPS: {:.2}, Frame Time: {:.2}ms", rate, time);
    }
}
