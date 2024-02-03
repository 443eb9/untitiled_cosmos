use bevy::{app::App, DefaultPlugins};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, untitled_lib::CosmosPlugin))
        .run();
}
