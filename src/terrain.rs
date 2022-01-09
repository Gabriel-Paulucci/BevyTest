use noise::{
    utils::{NoiseMapBuilder, PlaneMapBuilder},
    Fbm, Perlin,
};

pub fn gen_perlin_noise() {
    let perlin = Fbm::default();

    PlaneMapBuilder::new(&perlin)
        .set_size(2048, 2048)
        .set_x_bounds(-5., 5.)
        .set_y_bounds(-5., 5.)
        .build()
        .write_to_file("sla.png");
}
