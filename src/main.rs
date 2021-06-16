mod snake;

use bevy::prelude::*;
use snake::SnakePlugin;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 500.0,
            height: 500.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(SnakePlugin)
        .run();
}
