use bevy::prelude::*;

use super::AppState;

const SELECTED_COLOR: Color = Color::rgb(0.6, 0.6, 0.6);
const OPTION_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);

#[derive(Component, Debug)]
struct OptionText;

#[derive(Bundle)]
struct OptionTextBundle {
    lable: OptionText,
    #[bundle]
    text: Text2dBundle,
}

pub struct OptionPlugin;
impl Plugin for OptionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenuState).with_system(spawn_options))
            .add_system_set(
                SystemSet::on_exit(AppState::MainMenuState).with_system(destroy_Options),
            );
    }
}

fn spawn_options(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("Open_Sans/OpenSans-ExtraBold.ttf");
    let text_style_selected = TextStyle {
        font: font.clone(),
        font_size: 50.0,
        color: SELECTED_COLOR,
    };

    let text_style_option = TextStyle {
        font,
        font_size: 40.0,
        color: OPTION_COLOR,
    };

    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    commands.spawn_bundle(OptionTextBundle {
        lable: OptionText,
        text: Text2dBundle {
            text: Text::with_section("- PLAY -", text_style_selected.clone(), text_alignment),
            transform: Transform::from_xyz(0.0, 15.0, 10.0),
            ..Default::default()
        },
    });
    commands.spawn_bundle(OptionTextBundle {
        lable: OptionText,
        text: Text2dBundle {
            text: Text::with_section("QUIT", text_style_option.clone(), text_alignment),
            transform: Transform::from_xyz(0.0, -45.0, 10.0),
            ..Default::default()
        },
    });
}

fn destroy_Options(mut commands: Commands, query: Query<Entity, With<OptionText>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn();
    }
}
