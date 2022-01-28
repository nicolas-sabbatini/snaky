use super::arena::Position;
use bevy::{core::FixedTimestep, prelude::*};

// Snake constant
const HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const BODY_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

// Components and Bundles definitions
#[derive(Component, Debug)]
struct Head;

#[derive(Bundle)]
struct HeadBundle {
    head: Head,
    position: Position,
    movement_status: MovementStatus,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}
impl Into<Position> for Direction {
    fn into(self) -> Position {
        match self {
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Down => Position { x: 0, y: 1 },
        }
    }
}

#[derive(Component, Debug)]
struct MovementStatus {
    current_direction: Direction,
    next_direction: Direction,
    buffer_direction: Option<Direction>,
}

// Plugin definition
pub struct SnakePlugin;
impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_head);

        app.add_system(
            handle_input
                .label(SnakeStages::Input)
                .before(SnakeStages::Movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(move_snake.label(SnakeStages::Movement)),
        );
    }
}

// System definitions
#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SnakeStages {
    Input,
    Movement,
    Collision,
    Growth,
}

fn spawn_head(mut commands: Commands) {
    commands.spawn_bundle(HeadBundle {
        head: Head,
        position: Position { x: 0, y: 0 },
        movement_status: MovementStatus {
            current_direction: Direction::Right,
            next_direction: Direction::Right,
            buffer_direction: None,
        },
        sprite: SpriteBundle {
            sprite: Sprite {
                color: HEAD_COLOR,
                custom_size: Some(Vec2::new(95.0, 95.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    });
}

fn handle_input(mut query: Query<&mut MovementStatus, With<Head>>, key_input: Res<Input<KeyCode>>) {
    let mut target_direction: Vec<Direction> = Vec::new();
    if key_input.pressed(KeyCode::Left) {
        target_direction.push(Direction::Left);
    }
    if key_input.pressed(KeyCode::Right) {
        target_direction.push(Direction::Right);
    }
    if key_input.pressed(KeyCode::Up) {
        target_direction.push(Direction::Up);
    }
    if key_input.pressed(KeyCode::Down) {
        target_direction.push(Direction::Down);
    }

    for td in target_direction.iter() {
        for mut movement_status in query.iter_mut() {
            if movement_status.current_direction.opposite() != *td {
                movement_status.next_direction = *td;
                movement_status.buffer_direction = None;
            } else if movement_status.next_direction.opposite() != *td {
                movement_status.buffer_direction = Some(*td);
            }
        }
    }
}

fn move_snake(mut query: Query<(&mut MovementStatus, &mut Position), With<Head>>) {
    for (mut ms, mut pos) in query.iter_mut() {
        let step_direction: Position = ms.next_direction.into();
        if let Some(d) = ms.buffer_direction {
            ms.next_direction = d;
            ms.buffer_direction = None;
        }
        ms.current_direction = ms.next_direction;
        *pos = *pos + step_direction;
    }
}
