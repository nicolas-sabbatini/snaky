use arena::ArenaPlugin;
use bevy::prelude::*;
use snake::SnakePlugin;

mod arena;
mod snake;

const WIN_WIDTH: f32 = 800.0;
const WIN_HEIGHT: f32 = 600.0;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Bevy Invasion".to_string(),
        width: WIN_WIDTH,
        height: WIN_HEIGHT,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)));

    app.add_plugins(DefaultPlugins)
        .add_plugin(SnakePlugin)
        .add_plugin(ArenaPlugin);

    app.add_startup_system(setup_camera);

    app.run();
}

fn setup_camera(mut commands: Commands, win_res: Res<Windows>) {
    // Get primary window
    let win = win_res.get_primary().unwrap();
    // Create camera
    let mut new_camera = OrthographicCameraBundle::new_2d();
    new_camera.orthographic_projection.scaling_mode =
        bevy::render::camera::ScalingMode::FixedVertical;
    // Set camera variables
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
