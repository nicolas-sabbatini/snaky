use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use play_state::PlayStatePlugin;

mod play_state;

const WIN_WIDTH: f32 = 800.0;
const WIN_HEIGHT: f32 = 600.0;

#[derive(Debug)]
struct GameSize {
    width: f32,
    height: f32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenuState,
    PlayState,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Bevy Invasion".to_string(),
        width: WIN_WIDTH,
        height: WIN_HEIGHT,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    .insert_resource(GameSize {
        width: WIN_WIDTH,
        height: WIN_HEIGHT,
    });

    app.add_state(AppState::MainMenuState); // PlayState);

    app.add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default());

    app.add_plugins(DefaultPlugins).add_plugin(PlayStatePlugin);

    app.add_startup_system(setup_camera)
        .add_startup_system(setup_ui_camera);

    app.run();
}

fn setup_camera(mut commands: Commands, win_res: Res<Windows>) {
    // Get primary window
    let win = win_res.get_primary().unwrap();
    // Create camera
    let mut new_camera = OrthographicCameraBundle::new_2d();
    // Set camera variables
    new_camera.orthographic_projection.scaling_mode =
        bevy::render::camera::ScalingMode::FixedVertical;
    new_camera.orthographic_projection.scale = win.height() / 2.0;
    // Spawn new camera
    commands.spawn_bundle(new_camera);
    // Spawn letterboxing sprites
    let mut spawn_letterboxing = |x_mul: f32| {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(win.width(), win.height())),
                ..Default::default()
            },
            transform: Transform::from_xyz(win.width() * x_mul, 0.0, 99.0),
            ..Default::default()
        });
    };
    spawn_letterboxing(1.0);
    spawn_letterboxing(-1.0);
}

fn setup_ui_camera(mut commands: Commands, win_res: Res<Windows>) {
    // Get primary window
    let win = win_res.get_primary().unwrap();
    // Create camera
    let mut new_camera = UiCameraBundle::default();
    // Set camera variables
    new_camera.orthographic_projection.scaling_mode =
        bevy::render::camera::ScalingMode::FixedVertical;
    new_camera.orthographic_projection.window_origin = bevy::render::camera::WindowOrigin::Center;

    new_camera.orthographic_projection.scale = win.height() / 2.0;
    // Spawn new camera
    commands.spawn_bundle(new_camera);
}
