pub mod terrain;

use bevy::{prelude::*, DefaultPlugins};
use terrain::gen_perlin_noise;

fn main() {
    // App::new()
    //     .insert_resource(Msaa { samples: 4 })
    //     .add_plugins(DefaultPlugins)
    //     .run();

    gen_perlin_noise();
}
