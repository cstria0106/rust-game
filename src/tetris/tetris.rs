pub mod tetromino;
use bevy::prelude::*;

use self::tetromino::Tetromino;

pub struct TetrisPlugin;

impl Plugin for TetrisPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let mut mino = Tetromino::t();
        println!("Clockwise");
        for _ in 0..4 {
            println!("{}\n----", mino);
            mino.turn_clockwise();
        }

        println!("Counterclockwise");
        for _ in 0..4 {
            println!("{}\n----", mino);
            mino.turn_counterclockwise();
        }
    }
}
