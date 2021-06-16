use bevy::prelude::*;
use rand::prelude::*;
use std::ops::*;

pub const FIELD_WIDTH: u32 = 6;
pub const FIELD_HEIGHT: u32 = 6;

struct Gameover(bool);

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct LastDirection(Direction);

impl Direction {
    fn delta(&self) -> Position {
        match self {
            Direction::Up => Position { x: 0, y: 1 },
            Direction::Down => Position { x: 0, y: -1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

struct SnakeHead(i32);

struct SnakeTail(Entity);

struct Apple;

struct Materials {
    head: Handle<ColorMaterial>,
    tail: Handle<ColorMaterial>,
    apple: Handle<ColorMaterial>,
}

struct LastSnakePart(Option<Entity>);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Position {
    x: i16,
    y: i16,
}

impl From<&LastPosition> for Position {
    fn from(last_position: &LastPosition) -> Self {
        Position {
            x: last_position.0.x,
            y: last_position.0.y,
        }
    }
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

#[derive(Clone)]
struct LastPosition(Position);

impl From<&Position> for LastPosition {
    fn from(pos: &Position) -> Self {
        return LastPosition(pos.clone());
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

struct MovementTimer(Timer);

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        head: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        tail: materials.add(Color::rgb(0.6, 0.6, 0.6).into()),
        apple: materials.add(Color::rgb(1.0, 0.1, 0.1).into()),
    });
    commands.insert_resource(MovementTimer(Timer::from_seconds(0.5, true)));
    commands.insert_resource(LastSnakePart(None));
    commands.insert_resource(Gameover(false));
}

fn spawn_snake_head(
    mut commands: Commands,
    materials: Res<Materials>,
    mut last_snake_part: ResMut<LastSnakePart>,
) {
    let snake_initial_position = Position { x: 0, y: 0 };

    last_snake_part.0 = Some(
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.head.clone(),
                sprite: Sprite::new(Vec2::new(1.0, 1.0)),
                ..Default::default()
            })
            .insert(SnakeHead(0))
            .insert(snake_initial_position.clone())
            .insert(LastPosition::from(&snake_initial_position))
            .insert(Size::square(1.0))
            .insert(Direction::Right)
            .insert(LastDirection(Direction::Right))
            .id(),
    );

    commands.insert_resource(LastSnakePart)
}

fn spawn_apple(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.apple.clone(),
            sprite: Sprite::new(Vec2::new(1.0, 1.0)),
            ..Default::default()
        })
        .insert(Apple)
        .insert(Position { x: 1, y: 1 })
        .insert(Size::square(1.0));
}

fn move_apple(
    mut apple_positions: Query<
        &mut Position,
        (With<Apple>, (Without<SnakeHead>, Without<SnakeTail>)),
    >,
    snake_heads: Query<(&Position, &SnakeHead), (Without<Apple>, Without<SnakeTail>)>,
    snake_tails: Query<&Position, (With<SnakeTail>, Without<Apple>, Without<SnakeHead>)>,
    mut grow_event: EventReader<GrowEvent>,
) {
    let mut apple_position = apple_positions
        .single_mut()
        .expect("there should be exactly one apple in the game");

    let snake_head = snake_heads
        .single()
        .expect("there should be exactly one snake head in the game");

    if grow_event.iter().next().is_some() {
        move_apple_randomly(
            &mut apple_position,
            snake_head.0,
            &snake_tails.iter().collect(),
        );
    }
}

fn check_apple_collision(
    apple_positions: Query<&Position, (With<Apple>, Without<SnakeHead>)>,
    snake_heads: Query<(&Position, &SnakeHead), Without<Apple>>,
    mut grow_event: EventWriter<GrowEvent>,
) {
    let apple_position = apple_positions
        .single()
        .expect("there should be exactly one apple in the game");

    let snake_head = snake_heads
        .single()
        .expect("there should be exactly one snake head in the game");

    if *apple_position == *snake_head.0 {
        grow_event.send(GrowEvent);
    }
}

struct GrowEvent;
fn snake_grow(
    mut commands: Commands,
    mut grow_event: EventReader<GrowEvent>,
    materials: Res<Materials>,
    mut last_snake_part: ResMut<LastSnakePart>,
    last_positions: Query<&LastPosition>,
    mut snake_head: Query<&mut SnakeHead>,
) {
    let last_snake_part_position = last_positions.get(last_snake_part.0.unwrap()).unwrap();

    if grow_event.iter().next().is_some() {
        snake_head
            .single_mut()
            .expect("snake head must be unique in the game")
            .0 += 1;

        last_snake_part.0 = Some(
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.tail.clone(),
                    sprite: Sprite::new(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                })
                .insert(SnakeTail(last_snake_part.0.unwrap()))
                .insert(Position::from(last_snake_part_position))
                .insert(Size::square(0.9))
                .insert(last_snake_part_position.clone())
                .id(),
        );
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite), Changed<Size>>) {
    let window = windows.get_primary().unwrap();
    for (size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            (size.width / FIELD_WIDTH as f64 * window.width() as f64) as f32,
            (size.height / FIELD_HEIGHT as f64 * window.height() as f64) as f32,
        )
    }
}

fn position_translation(
    windows: Res<Windows>,
    mut q: Query<(&Position, &mut Transform), Changed<Position>>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.0) + (tile_size / 2.0)
    }

    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, FIELD_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, FIELD_HEIGHT as f32),
            0.0,
        );
    }
}

struct MovementTickEvent;

fn movement_ticker(
    mut event: EventWriter<MovementTickEvent>,
    mut last_positions: Query<(&Position, &mut LastPosition)>,
    mut last_directions: Query<(&Direction, &mut LastDirection)>,
    mut timer: ResMut<MovementTimer>,
    time: Res<Time>,
    gameover: Res<Gameover>,
) {
    if gameover.0 {
        return;
    }

    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        event.send(MovementTickEvent);

        for (position, mut last_position) in last_positions.iter_mut() {
            *last_position = LastPosition::from(position);
        }

        for (direction, mut last_direction) in last_directions.iter_mut() {
            *last_direction = LastDirection(direction.clone());
        }
    }
}

fn head_movement(
    mut event: EventReader<MovementTickEvent>,
    mut head: Query<(&mut Position, &mut Direction, &LastDirection, &SnakeHead)>,
    keyboard: Res<Input<KeyCode>>,
) {
    for pressed in keyboard.get_just_pressed() {
        for (_, mut direction, last_direction, head) in head.iter_mut() {
            let input_direction = match pressed {
                KeyCode::Left => Direction::Left,
                KeyCode::Right => Direction::Right,
                KeyCode::Down => Direction::Down,
                KeyCode::Up => Direction::Up,
                _ => continue,
            };

            if head.0 != 0
                && (input_direction == last_direction.0
                    || input_direction == last_direction.0.opposite())
            {
                continue;
            }

            *direction = input_direction;
        }
    }

    for _ in event.iter() {
        for (mut position, direction, _, _) in head.iter_mut() {
            *position += direction.delta();
        }
    }
}

fn tail_movement(
    mut event: EventReader<MovementTickEvent>,
    mut tail: Query<(&mut Position, &SnakeTail)>,
    last_positions: Query<&LastPosition>,
) {
    for _ in event.iter() {
        for (mut position, following) in tail.iter_mut() {
            *position = Position::from(last_positions.get(following.0).unwrap())
        }
    }
}

fn check_gameover(
    mut gameover: ResMut<Gameover>,
    mut head_positions: Query<
        (&mut Position, &LastPosition),
        (With<SnakeHead>, Without<SnakeTail>),
    >,
    mut tail_positions: Query<
        (&mut Position, &LastPosition),
        (With<SnakeTail>, Without<SnakeHead>),
    >,
) {
    if gameover.0 {
        return;
    }

    for (head_position, _) in head_positions.iter_mut() {
        if !(0..FIELD_WIDTH as i16).contains(&head_position.x)
            || !(0..FIELD_HEIGHT as i16).contains(&head_position.y)
        {
            gameover.0 = true;
            break;
        }

        for (tail_position, _) in tail_positions.iter_mut() {
            if *head_position == *tail_position {
                gameover.0 = true;
                break;
            }
        }

        if gameover.0 {
            break;
        }
    }

    if gameover.0 {
        for (mut head_position, last_position) in head_positions.iter_mut() {
            *head_position = Position::from(last_position);
        }

        for (mut tail_position, last_position) in tail_positions.iter_mut() {
            *tail_position = Position::from(last_position);
        }
    }
}

fn move_apple_randomly(
    apple_position: &mut Position,
    head_position: &Position,
    tail_positions: &Vec<&Position>,
) {
    let mut possible_positions: Vec<Position> =
        Vec::with_capacity((FIELD_WIDTH * FIELD_HEIGHT) as usize);

    for x in 0..FIELD_WIDTH as i16 {
        for y in 0..FIELD_HEIGHT as i16 {
            if x == head_position.x && y == head_position.y {
                continue;
            }

            if tail_positions
                .iter()
                .any(|position| -> bool { position.x == x && position.y == y })
            {
                continue;
            }

            possible_positions.push(Position { x, y });
        }
    }

    *apple_position = possible_positions
        .choose(&mut rand::thread_rng())
        .unwrap()
        .clone();
}

pub struct SnakePlugin;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, SystemLabel)]
enum MovementSystem {
    Ticker,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, SystemLabel)]
enum SnakeSystem {
    Movement,
    AppleCollision,
    SnakeGrow,
}

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let movement_systems = SystemSet::new()
            .with_system(movement_ticker.system().label(MovementSystem::Ticker))
            .with_system(head_movement.system().after(MovementSystem::Ticker))
            .with_system(tail_movement.system().after(MovementSystem::Ticker));

        app.add_startup_system(setup.system())
            .add_startup_stage(
                "game_setup",
                SystemStage::parallel()
                    .with_system(spawn_snake_head.system())
                    .with_system(spawn_apple.system()),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_system(position_translation.system())
                    .with_system(size_scaling.system()),
            )
            .add_system_set(movement_systems.label(SnakeSystem::Movement))
            .add_system(
                check_apple_collision
                    .system()
                    .label(SnakeSystem::AppleCollision)
                    .after(SnakeSystem::Movement),
            )
            .add_system(
                snake_grow
                    .system()
                    .label(SnakeSystem::SnakeGrow)
                    .after(SnakeSystem::AppleCollision),
            )
            .add_system(move_apple.system().before(SnakeSystem::SnakeGrow))
            .add_system(check_gameover.system().after(SnakeSystem::Movement))
            .add_event::<GrowEvent>()
            .add_event::<MovementTickEvent>();
    }
}
