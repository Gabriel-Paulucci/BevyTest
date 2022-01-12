pub mod camera;
pub mod terrain;

use bevy::{prelude::*, DefaultPlugins};
use camera::{camera_controller, CameraController};
use terrain::gen_terrain;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(gen_terrain)
        .add_system(camera_controller)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-1.0, 500.0, 500.0)
                .looking_at(Vec3::new(-1.0, 1.0, 0.0), Vec3::Y),
            ..Default::default()
        })
        .insert(CameraController::default());
}
