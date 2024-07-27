mod camera;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // Camera
        .add_systems(Startup, camera::setup)
        .add_systems(Update, camera::update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight { ..default() },
        transform: Transform::from_xyz(-4.0, -8.0, -4.0),
        ..default()
    });
}
