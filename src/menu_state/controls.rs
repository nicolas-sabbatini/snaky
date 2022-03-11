use bevy::prelude::*;

use super::AppState;

const CONTROLS_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

#[derive(Component, Debug)]
struct ControlText;

#[derive(Bundle)]
struct ControlTextBundle {
    lable: ControlText,
    #[bundle]
    text: Text2dBundle,
}

pub struct ControlPlugin;
impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenuState).with_system(spawn_control))
            .add_system_set(
                SystemSet::on_exit(AppState::MainMenuState).with_system(destroy_control),
            );
    }
}

fn spawn_control(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("Open_Sans/OpenSans-ExtraBold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 30.0,
        color: CONTROLS_COLOR,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    commands.spawn_bundle(ControlTextBundle {
        lable: ControlText,
        text: Text2dBundle {
            text: Text::with_section("Use arrows to move.", text_style.clone(), text_alignment),
            transform: Transform::from_xyz(0.0, -200.0, 10.0),
            ..Default::default()
        },
    });
    commands.spawn_bundle(ControlTextBundle {
        lable: ControlText,
        text: Text2dBundle {
            text: Text::with_section(
                "Use 'Space' to Select/Pause.",
                text_style.clone(),
                text_alignment,
            ),
            transform: Transform::from_xyz(0.0, -230.0, 10.0),
            ..Default::default()
        },
    });
}

fn destroy_control(mut commands: Commands, query: Query<Entity, With<ControlText>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn();
    }
}
