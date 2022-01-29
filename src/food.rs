use crate::arena::{ArenaSize, Position, CEL_SIZE};
use bevy::{
    ecs::{schedule::ShouldRun, system::QuerySingleError},
    prelude::*,
};
use rand::prelude::random;

const FOOD_COLOR: Color = Color::rgb(0.7, 0.0, 0.0);

#[derive(Component, Debug)]
struct Food;

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
                .with_run_criteria(no_food)
                .with_system(food_spawner),
        );
    }
}

fn food_spawner(mut commands: Commands, arena_size: Res<ArenaSize>) {
    commands.spawn_bundle(FoodBundle {
        food: Food,
        position: Position {
            x: (random::<f32>() * arena_size.width as f32) as i32,
            y: (random::<f32>() * arena_size.height as f32) as i32,
        },
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
