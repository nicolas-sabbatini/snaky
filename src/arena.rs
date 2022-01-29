use crate::GameSize;
use bevy::prelude::*;
use std::ops::Add;

const ARENA_WIDTH: u32 = 40;
const ARENA_HEIGHT: u32 = 30;
pub const CEL_SIZE: f32 = 20.0;

pub struct ArenaSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
impl Add for Position {
    type Output = Position;
    fn add(self, other: Self) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub struct ArenaPlugin;
impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_arena);

        app.add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new().with_system(update_position),
        );
    }
}

fn setup_arena(mut commands: Commands) {
    commands.insert_resource(ArenaSize {
        width: ARENA_WIDTH,
        height: ARENA_HEIGHT,
    })
}

fn update_position(
    arena_size: Res<ArenaSize>,
    game_size: Res<GameSize>,
    mut query: Query<(&Position, &mut Transform)>,
) {
    fn convert(pos: f32, g_size: f32, a_size: f32) -> f32 {
        pos / a_size * g_size - (g_size / 2.0) + (CEL_SIZE * 0.5)
    }
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, game_size.width, arena_size.width as f32),
            -convert(pos.y as f32, game_size.height, arena_size.height as f32),
            10.0,
        );
    }
}
