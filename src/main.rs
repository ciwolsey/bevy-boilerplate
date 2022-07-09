use bevy::prelude::*;
use rand::{thread_rng, Rng};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Grid".into(),
            width: 750.0,
            height: 750.0,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    // Bevy 0.7
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Bevy 0.8
    // commands.spawn_bundle(Camera2dBundle::default());
}