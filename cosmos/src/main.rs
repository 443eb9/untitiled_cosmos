use bevy::{
    app::{App, PluginGroup},
    window::{PresentMode, Window, WindowPlugin},
    DefaultPlugins,
};
use untitled_lib::CosmosPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::Immediate,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            CosmosPlugin,
        ))
        .run();
}
