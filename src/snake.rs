use crate::arena::CEL_SIZE;

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

#[derive(Component, Debug)]
struct BodyPart;

#[derive(Bundle)]
struct BodyPartBundle {
    body_part: BodyPart,
    position: Position,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Component, Debug)]
struct Last;

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
        app.add_startup_system(spawn_head)
            .add_startup_system(spawn_body);

        app.add_system(
            handle_input
                .label(SnakeStages::Input)
                .before(SnakeStages::Movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.125))
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
        position: Position { x: 1, y: 0 },
        movement_status: MovementStatus {
            current_direction: Direction::Right,
            next_direction: Direction::Right,
            buffer_direction: None,
        },
        sprite: SpriteBundle {
            sprite: Sprite {
                color: HEAD_COLOR,
                custom_size: Some(Vec2::new(CEL_SIZE * 0.95, CEL_SIZE * 0.95)),
                ..Default::default()
            },
            ..Default::default()
        },
    });
}

fn spawn_body(mut commands: Commands) {
    commands
        .spawn_bundle(BodyPartBundle {
            body_part: BodyPart,
            position: Position { x: 0, y: 0 },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: BODY_COLOR,
                    custom_size: Some(Vec2::new(CEL_SIZE * 0.75, CEL_SIZE * 0.75)),
                    ..Default::default()
                },
                ..Default::default()
            },
        })
        .insert(Last);
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

fn move_snake(
    mut query: QuerySet<(
        QueryState<(&mut MovementStatus, &mut Position), With<Head>>,
        QueryState<&mut Position, With<BodyPart>>,
    )>,
) {
    let head_prev_pos: Position;
    // Handle head movement
    match query.q0().get_single_mut() {
        Ok((mut head_ms, mut head_pos)) => {
            head_prev_pos = (*head_pos).clone();
            // Move head
            let step_direction: Position = head_ms.next_direction.into();
            if let Some(d) = head_ms.buffer_direction {
                head_ms.next_direction = d;
                head_ms.buffer_direction = None;
            }
            head_ms.current_direction = head_ms.next_direction;
            *head_pos = *head_pos + step_direction;
        }
        Err(_) => panic!("HOW DID WE EVEN GET HERE!?!?"),
    };
    // Handle body movement
    // Create a vector to save positions to move the body
    let mut nex_body_pos: Vec<Position> = query.q1().iter().skip(1).map(|p| *p).collect();
    nex_body_pos.push(head_prev_pos);
    for (mut bp, next) in query.q1().iter_mut().zip(nex_body_pos.iter()) {
        *bp = *next;
    }
}
