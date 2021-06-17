#[path = "tetris/tetris.rs"]
mod tetris;

use bevy::prelude::*;
use tetris::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 500.0,
            height: 500.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TetrisPlugin)
        .run();
}
