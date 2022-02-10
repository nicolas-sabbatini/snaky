use bevy::prelude::*;

use arena::ArenaPlugin;
use food::FoodPlugin;
use snake::SnakePlugin;
use score_board::ScoreBoardPlugin;

mod arena;
mod food;
mod snake;
mod score_board;

pub struct PlayStatePlugin;
impl Plugin for PlayStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SnakePlugin)
            .add_plugin(ArenaPlugin)
            .add_plugin(FoodPlugin)
            .add_plugin(ScoreBoardPlugin);
    }
}
