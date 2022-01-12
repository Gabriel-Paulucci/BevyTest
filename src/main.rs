pub mod terrain;

use bevy::{prelude::*, DefaultPlugins};
use terrain::gen_terrain;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(gen_terrain)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 150.5, 150.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
