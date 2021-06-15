use std::ops::{Add, AddAssign};

use bevy::prelude::*;

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

struct SnakeHead;
struct Materials {
    head_material: Handle<ColorMaterial>,
}

#[derive(Debug)]
struct Position {
    x: i16,
    y: i16,
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

struct Size {
    width: f64,
    height: f64,
}

impl Size {
    fn square(size: f64) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    })
}

fn spawn_snake(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.head_material.as_weak(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            (size.width / ARENA_WIDTH as f64 * window.width() as f64) as f32,
            (size.height / ARENA_HEIGHT as f64 * window.height() as f64) as f32,
        )
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.0) + (tile_size / 2.0)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn snake_movement(
    mut head_positions: Query<(&SnakeHead, &mut Position)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut delta = Position { x: 0, y: 0 };

    for pressed in keyboard_input.get_pressed() {
        &mut delta += match pressed {
            KeyCode::Left => Position { x: -2, y: 0 },
            KeyCode::Right => Position { x: 2, y: 0 },
            KeyCode::Down => Position { x: 0, y: -2 },
            KeyCode::Up => Position { x: 0, y: 2 },
            _ => Position { x: 0, y: 0 },
        }
    }

    for (_head, mut position) in head_positions.iter_mut() {
        position.add_assign(delta);
    }
}

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_snake.system()))
        .add_system(snake_movement.system())
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation.system())
                .with_system(size_scaling.system()),
        )
        .add_plugins(DefaultPlugins)
        .run();
}
