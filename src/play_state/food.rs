use super::{
    arena::{ArenaSize, Position, CEL_SIZE},
    is_in_play_state_chain,
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
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(no_food.chain(is_in_play_state_chain))
                .with_system(food_spawner),
        );
    }
}

fn food_spawner(mut commands: Commands, arena_size: Res<ArenaSize>, query: Query<&Position>) {
    let new_position = || Position {
        x: (random::<f32>() * arena_size.width as f32) as i32,
        y: (random::<f32>() * arena_size.height as f32) as i32,
    };
    let mut position = new_position();
    let mut ocupided_position: Vec<Position> = Vec::new();
    for pos in query.iter() {
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

fn no_food(query: Query<&Food>) -> ShouldRun {
    match query.get_single() {
        Ok(_) => ShouldRun::No,
        Err(QuerySingleError::NoEntities(_)) => ShouldRun::Yes,
        Err(QuerySingleError::MultipleEntities(_)) => panic!("HOW DID WE EVEN GET HERE!?!?"),
    }
}
