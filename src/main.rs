pub mod terrain;

use bevy::{prelude::*, DefaultPlugins};
use terrain::gen_terrain;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(gen_terrain)
        .run();
}
