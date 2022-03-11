use bevy::prelude::*;

use super::AppState;

const TITLE_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component, Debug)]
struct TitleText;

#[derive(Bundle)]
struct TitleTextBundle {
    lable: TitleText,
    #[bundle]
    text: Text2dBundle,
}

pub struct TitlePlugin;
impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenuState).with_system(spawn_title))
            .add_system_set(SystemSet::on_exit(AppState::MainMenuState).with_system(destroy_title));
    }
}

fn spawn_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("Open_Sans/OpenSans-ExtraBold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 150.0,
        color: TITLE_COLOR,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    commands.spawn_bundle(TitleTextBundle {
        lable: TitleText,
        text: Text2dBundle {
            text: Text::with_section("SNAKY!", text_style.clone(), text_alignment),
            transform: Transform::from_xyz(0.0, 175.0, 10.0),
            ..Default::default()
        },
    });
}

fn destroy_title(mut commands: Commands, query: Query<Entity, With<TitleText>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn();
    }
}
