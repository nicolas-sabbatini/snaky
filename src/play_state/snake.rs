use super::arena::Position;
use super::food::spawn_food;
use super::is_in_play_state_chain;
use super::{
    arena::{ARENA_HEIGHT, ARENA_WIDTH, CEL_SIZE},
    food::Food,
    AppState,
};
use bevy::{core::FixedTimestep, ecs::schedule::ShouldRun, prelude::*};

// Snake constant
const HEAD_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const BODY_COLOR: Color = Color::rgb(0.6, 0.6, 0.6);

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

#[derive(Component, Debug)]
struct Order(usize);

#[derive(Debug)]
pub struct AmountBodyParts(pub usize);

#[derive(Bundle)]
struct BodyPartBundle {
    body_part: BodyPart,
    position: Position,
    order: Order,
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

// Events definitions
struct EatEvent;

struct GameOver(usize);

// Plugin definition
pub struct SnakePlugin;
impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EatEvent>();
        app.add_event::<GameOver>();

        app.add_system_set(
            SystemSet::on_enter(AppState::PlayState)
                .with_system(spawn_head)
                .with_system(spawn_body),
        )
        .add_system_set(
            SystemSet::on_update(AppState::PlayState).with_system(
                handle_input
                    .label(SnakeStages::Input)
                    .before(SnakeStages::Movement),
            ),
        )
        .add_system_set(
            SystemSet::new()
                .label(GameStages::Update)
                .with_run_criteria(FixedTimestep::step(0.125).chain(is_in_play_state_chain))
                .with_system(movement.label(SnakeStages::Movement))
                .with_system(eat.label(SnakeStages::Eat).after(SnakeStages::Movement))
                .with_system(grow.label(SnakeStages::Grow).after(SnakeStages::Eat))
                .with_system(
                    collision
                        .label(SnakeStages::Collision)
                        .after(SnakeStages::Movement),
                ),
        )
        .add_system_set(
            SystemSet::new()
                .label(GameStages::EndGame)
                .after(GameStages::Update)
                .with_run_criteria(game_over.chain(is_in_play_state_chain))
                .with_system(clear.label(GameOverStages::Clear))
                .with_system(
                    spawn_body
                        .label(GameOverStages::RespawnBody)
                        .after(GameOverStages::Clear),
                )
                .with_system(
                    spawn_head
                        .label(GameOverStages::RespawnHead)
                        .after(GameOverStages::Clear),
                )
                .with_system(
                    spawn_food
                        .label(GameOverStages::RespawnFood)
                        .after(GameOverStages::Clear),
                ),
        );
    }
}

// System definitions
#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameStages {
    Update,
    EndGame,
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SnakeStages {
    Input,
    Movement,
    Eat,
    Grow,
    Collision,
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameOverStages {
    Clear,
    RespawnBody,
    RespawnHead,
    RespawnFood,
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
    commands.spawn_bundle(BodyPartBundle {
        body_part: BodyPart,
        position: Position { x: 0, y: 0 },
        order: Order(1),
        sprite: SpriteBundle {
            sprite: Sprite {
                color: BODY_COLOR,
                custom_size: Some(Vec2::new(CEL_SIZE * 0.75, CEL_SIZE * 0.75)),
                ..Default::default()
            },
            ..Default::default()
        },
    });
    commands.insert_resource(AmountBodyParts(1));
}

// This is buggy as hell TODO fix
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
            if movement_status.current_direction.opposite() != *td
                && movement_status.current_direction != *td
            {
                movement_status.next_direction = *td;
                movement_status.buffer_direction = None;
            } else if movement_status.next_direction.opposite() != *td {
                movement_status.buffer_direction = Some(*td);
            }
        }
    }
}

// This is buggy as hell TODO fix
fn movement(
    amount_body_parts: Res<AmountBodyParts>,
    mut query: QuerySet<(
        QueryState<(&mut MovementStatus, &mut Position), With<Head>>,
        QueryState<(&mut Position, &Order), With<BodyPart>>,
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
        Err(_) => panic!("HOW DID WE EVEN GET HERE!?!? No head error"),
    };
    // Handle body movement
    // Create a vector to save positions to move the body
    let mut nex_body_pos: Vec<Position> = vec![head_prev_pos; amount_body_parts.0];
    for (bp, order) in query.q1().iter() {
        if order.0 < amount_body_parts.0 {
            nex_body_pos[order.0] = *bp;
        }
    }
    for (mut bp, order) in query.q1().iter_mut() {
        *bp = nex_body_pos[order.0 - 1];
    }
}

fn eat(
    mut commands: Commands,
    mut event_writer: EventWriter<EatEvent>,
    food_query: Query<(Entity, &Position), With<Food>>,
    head_query: Query<&Position, With<Head>>,
) {
    let head_pos = match head_query.get_single() {
        Ok(pos) => pos,
        Err(_) => panic!("HOW DID WE EVEN GET HERE!?!? No head for food"),
    };
    match food_query.get_single() {
        Ok((ent, pos)) => {
            if pos == head_pos {
                commands.entity(ent).despawn();
                event_writer.send(EatEvent);
            }
        }
        Err(_) => panic!("HOW DID WE EVEN GET HERE!?!? No food for head"),
    };
}

fn grow(
    mut commands: Commands,
    mut event_reader: EventReader<EatEvent>,
    mut amount_body_parts: ResMut<AmountBodyParts>,
    body_query: Query<(&Position, &Order), With<BodyPart>>,
) {
    if !event_reader.iter().next().is_some() {
        return;
    }
    for (pos, order) in body_query.iter() {
        if order.0 == amount_body_parts.0 {
            amount_body_parts.0 += 1;
            commands.spawn_bundle(BodyPartBundle {
                body_part: BodyPart,
                position: *pos,
                order: Order(amount_body_parts.0),
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: BODY_COLOR,
                        custom_size: Some(Vec2::new(CEL_SIZE * 0.75, CEL_SIZE * 0.75)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            });
            break;
        }
    }
}

fn collision(
    mut event_writer: EventWriter<GameOver>,
    amount_body_parts: Res<AmountBodyParts>,
    body_query: Query<&Position, With<BodyPart>>,
    head_query: Query<&Position, With<Head>>,
) {
    let head_pos = match head_query.get_single() {
        Ok(pos) => pos,
        Err(_) => panic!("HOW DID WE EVEN GET HERE!?!? No head to collide"),
    };
    if head_pos.x < 0 || head_pos.y < 0 || head_pos.x >= ARENA_WIDTH || head_pos.y >= ARENA_HEIGHT {
        event_writer.send(GameOver(amount_body_parts.0 - 1));
        return;
    }
    for body_pos in body_query.iter() {
        if head_pos == body_pos {
            event_writer.send(GameOver(amount_body_parts.0 - 1));
            return;
        }
    }
}

fn game_over(mut event_reader: EventReader<GameOver>) -> ShouldRun {
    if event_reader.iter().next().is_some() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn clear(
    mut commands: Commands,
    body_query: Query<Entity, With<BodyPart>>,
    food_query: Query<Entity, With<Food>>,
    head_query: Query<Entity, With<Head>>,
) {
    for ent in body_query.iter() {
        commands.entity(ent).despawn();
    }
    for ent in food_query.iter() {
        commands.entity(ent).despawn();
    }
    for ent in head_query.iter() {
        commands.entity(ent).despawn();
    }
}
