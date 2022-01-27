use std::ops::Add;

use bevy::prelude::*;

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

struct ArenaSize {
    width: u32,
    height: u32,
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
    }
}

fn setup_arena(mut commands: Commands) {
    commands.insert_resource(ArenaSize {
        width: ARENA_WIDTH,
        height: ARENA_HEIGHT,
    })
}

fn update_position(arena: Res<ArenaSize>, mut query: Query<(&Position, &mut Transform)>) {}
