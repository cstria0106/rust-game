use bevy::prelude::*;

struct SnakeHead;
struct Materials {
    head_material: Handle<ColorMaterial>,
}

struct Position {
    x: i32,
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
        .insert(SnakeHead);
}

fn snake_movement(
    mut head_positions: Query<(&SnakeHead, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut delta = Vec3::ZERO;

    for pressed in keyboard_input.get_pressed() {
        delta += match pressed {
            KeyCode::Left => Vec3::X * -2.0,
            KeyCode::Right => Vec3::X * 2.0,
            KeyCode::Down => Vec3::Y * -2.0,
            KeyCode::Up => Vec3::Y * 2.0,
            _ => Vec3::ZERO,
        }
    }

    for (_head, mut transform) in head_positions.iter_mut() {
        transform.translation += delta;
    }
}

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_snake.system()))
        .add_system(snake_movement.system())
        .add_plugins(DefaultPlugins)
        .run();
}
