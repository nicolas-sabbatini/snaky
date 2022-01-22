use super::arena::Position;
use bevy::prelude::*;

// Snake constant
const HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const HEAD_BODY: Color = Color::rgb(0.5, 0.5, 0.5);

#[derive(Component, Debug)]
struct Head;

// Movement status
#[derive(Debug, PartialEq, Clone, Copy)]
enum DirectionStatus {
    Left,
    Right,
    Up,
    Down,
}
impl Into<Position> for DirectionStatus {
    fn into(self) -> Position {
        match self {
            DirectionStatus::Left => Position { x: -1, y: 0 },
            DirectionStatus::Right => Position { x: 1, y: 0 },
            DirectionStatus::Up => Position { x: 0, y: 1 },
            DirectionStatus::Down => Position { x: 0, y: -1 },
        }
    }
}

#[derive(Debug)]
struct Direction(DirectionStatus);

pub struct SnakePlugin;
impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Direction(DirectionStatus::Left));

        app.add_startup_system(spawn_head);

        app.add_system(move_head);
    }
}

fn spawn_head(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: HEAD_COLOR,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            ..Default::default()
        })
        .insert(Head)
        .insert(Position { x: 3, y: 3 });
}

fn move_head(
    mut query: Query<&mut Position, With<Head>>,
    mut dir: ResMut<Direction>,
    key_input: Res<Input<KeyCode>>,
) {
    if key_input.pressed(KeyCode::Left) && dir.0 != DirectionStatus::Right {
        dir.0 = DirectionStatus::Left;
    } else if key_input.pressed(KeyCode::Right) && dir.0 != DirectionStatus::Left {
        dir.0 = DirectionStatus::Right;
    } else if key_input.pressed(KeyCode::Up) && dir.0 != DirectionStatus::Down {
        dir.0 = DirectionStatus::Up;
    } else if key_input.pressed(KeyCode::Down) && dir.0 != DirectionStatus::Up {
        dir.0 = DirectionStatus::Down;
    }
    let step_direction: Position = dir.0.into();

    for mut pos in query.iter_mut() {
        *pos = *pos + step_direction;
    }
}
