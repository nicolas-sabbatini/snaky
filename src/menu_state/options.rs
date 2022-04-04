use bevy::app::AppExit;
use bevy::prelude::*;

use super::AppState;

const SELECTED_COLOR: Color = Color::rgb(0.6, 0.6, 0.6);
const OPTION_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);

#[derive(Bundle)]
struct OptionTextBundle {
    lable: OptionName,
    #[bundle]
    text: Text2dBundle,
}

#[derive(Component, Debug, PartialEq, Eq)]
enum OptionName {
    PLAY,
    QUIT,
}
impl OptionName {
    fn change(&mut self) {
        match self {
            OptionName::QUIT => *self = OptionName::PLAY,
            OptionName::PLAY => *self = OptionName::QUIT,
        }
    }
}

struct TextStylesResource {
    selected: TextStyle,
    option: TextStyle,
    text_alignment: TextAlignment,
}

pub struct OptionPlugin;
impl Plugin for OptionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenuState).with_system(spawn_options))
            .add_system_set(
                SystemSet::on_update(AppState::MainMenuState).with_system(update_options),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::MainMenuState).with_system(destroy_options),
            );
        println!("{:?}", OptionName::QUIT)
    }
}

fn spawn_options(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("Open_Sans/OpenSans-ExtraBold.ttf");

    let text_style = TextStylesResource {
        selected: TextStyle {
            font: font.clone(),
            font_size: 50.0,
            color: SELECTED_COLOR,
        },
        option: TextStyle {
            font,
            font_size: 40.0,
            color: OPTION_COLOR,
        },
        text_alignment: TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        },
    };

    commands.insert_resource(OptionName::PLAY);

    commands.spawn_bundle(OptionTextBundle {
        lable: OptionName::PLAY,
        text: Text2dBundle {
            text: Text::with_section(
                "- PLAY -",
                text_style.selected.clone(),
                text_style.text_alignment,
            ),
            transform: Transform::from_xyz(0.0, 15.0, 10.0),
            ..Default::default()
        },
    });
    commands.spawn_bundle(OptionTextBundle {
        lable: OptionName::QUIT,
        text: Text2dBundle {
            text: Text::with_section("QUIT", text_style.option.clone(), text_style.text_alignment),
            transform: Transform::from_xyz(0.0, -45.0, 10.0),
            ..Default::default()
        },
    });

    commands.insert_resource(text_style);
}

fn update_options(
    key_input: Res<Input<KeyCode>>,
    mut current_option: ResMut<OptionName>,
    text_styles: Res<TextStylesResource>,
    mut exit: EventWriter<AppExit>,
    mut app_state: ResMut<State<AppState>>,
    mut query: Query<(&mut Text, &OptionName)>,
) {
    if key_input.just_pressed(KeyCode::Up) || key_input.just_pressed(KeyCode::Down) {
        current_option.change();
        for (mut t, option_name) in query.iter_mut() {
            if *option_name == *current_option {
                *t = Text::with_section(
                    format!("- {:?} -", *option_name),
                    text_styles.selected.clone(),
                    text_styles.text_alignment,
                );
            } else {
                *t = Text::with_section(
                    format!("{:?}", *option_name),
                    text_styles.option.clone(),
                    text_styles.text_alignment,
                );
            }
        }
    }
    if key_input.just_pressed(KeyCode::Space) {
        match *current_option {
            OptionName::QUIT => exit.send(AppExit),
            OptionName::PLAY => app_state.set(AppState::PlayState).unwrap(),
        }
    }
}

fn destroy_options(mut commands: Commands, query: Query<Entity, With<OptionName>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn();
    }

    commands.remove_resource::<OptionName>();
    commands.remove_resource::<TextStylesResource>();
}
