use super::{
    arena::{ArenaSize, Position, CEL_SIZE},
    AppState,
};
use bevy::{
    ecs::{schedule::ShouldRun, system::QuerySingleError},
    prelude::*,
};
use rand::prelude::random;

const FOOD_COLOR: Color = Color::rgb(0.7, 0.0, 0.0);

#[derive(Component, Debug)]
pub struct Food;

#[derive(Bundle)]
struct FoodBundle {
    food: Food,
    position: Position,
    #[bundle]
    sprite: SpriteBundle,
}

pub struct FoodPlugin;
impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::PlayState).with_system(food_spawner));
    }
}

fn food_spawner(
    mut commands: Commands,
    arena_size: Res<ArenaSize>,
    pos_query: Query<&Position>,
    food_query: Query<&Food>,
) {
    match food_query.get_single() {
        // If there is food
        Ok(_) => (),
        // No food
        Err(QuerySingleError::NoEntities(_)) => spawn_food(commands, arena_size, pos_query),
        // Multiple foods
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("HOW DID WE EVEN GET HERE!?!? Multiple food error")
        }
    }
}

pub fn spawn_food(mut commands: Commands, arena_size: Res<ArenaSize>, pos_query: Query<&Position>) {
    let new_position = || Position {
        x: (random::<f32>() * arena_size.width as f32) as i32,
        y: (random::<f32>() * arena_size.height as f32) as i32,
    };
    let mut position = new_position();
    let mut ocupided_position: Vec<Position> = Vec::new();
    for pos in pos_query.iter() {
        ocupided_position.push(*pos);
    }
    while ocupided_position.contains(&position) {
        position = new_position();
    }
    commands.spawn_bundle(FoodBundle {
        food: Food,
        position,
        sprite: SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                custom_size: Some(Vec2::new(CEL_SIZE * 0.75, CEL_SIZE * 0.75)),
                ..Default::default()
            },
            ..Default::default()
        },
    });
}
