use super::AppState;
use bevy::{ecs::schedule::ShouldRun, prelude::*};

use arena::ArenaPlugin;
use food::FoodPlugin;
use score_board::ScoreBoardPlugin;
use snake::SnakePlugin;

mod arena;
mod food;
mod score_board;
mod snake;

pub struct PlayStatePlugin;
impl Plugin for PlayStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SnakePlugin)
            .add_plugin(ArenaPlugin)
            .add_plugin(FoodPlugin)
            .add_plugin(ScoreBoardPlugin);
    }
}

fn is_in_play_state_chain(In(input): In<ShouldRun>, state: Res<State<AppState>>) -> ShouldRun {
    if state.current() == &AppState::PlayState {
        input
    } else {
        ShouldRun::No
    }
}

fn is_in_play_state(state: Res<State<AppState>>) -> ShouldRun {
    if state.current() == &AppState::PlayState {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

