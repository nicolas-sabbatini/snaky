use super::AppState;
use bevy::{ecs::schedule::ShouldRun, prelude::*};
use controls::ControlPlugin;
use options::OptionPlugin;
use title::TitlePlugin;

mod controls;
mod options;
mod title;

pub struct MenuStatePlugin;
impl Plugin for MenuStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TitlePlugin)
            .add_plugin(ControlPlugin)
            .add_plugin(OptionPlugin);
    }
}

// Helper fn
fn is_in_main_menu_state_chain(In(input): In<ShouldRun>, state: Res<State<AppState>>) -> ShouldRun {
    if state.current() == &AppState::MainMenuState {
        input
    } else {
        ShouldRun::No
    }
}

fn is_in_main_menu_state(state: Res<State<AppState>>) -> ShouldRun {
    if state.current() == &AppState::MainMenuState {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
