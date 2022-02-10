use bevy::prelude::*;

use super::snake::AmountBodyParts;

const SCORE_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

pub struct ScoreBoardPlugin;
impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_scoreboard);

        app.add_system(change_score);
    }
}

fn spawn_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("Open_Sans/OpenSans-ExtraBold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 150.0,
        color: SCORE_COLOR,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section("0", text_style.clone(), text_alignment),
        ..Default::default()
    });
}

fn change_score(amount_body_parts: Res<AmountBodyParts>, mut query: Query<&mut Text>) {
    match query.get_single_mut() {
        Ok(mut text) => {
            text.sections[0].value = format!("{}", amount_body_parts.0 - 1);
        }
        Err(_) => panic!("HOW DID WE EVEN GET HERE!?!?"),
    }
}
