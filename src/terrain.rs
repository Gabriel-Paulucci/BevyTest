use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology::TriangleList,
    },
};
use noise::{
    utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder},
    Fbm,
};

#[derive(Component)]
pub struct Terrain;

fn gen_perlin_noise(width: usize, height: usize) -> NoiseMap {
    let noise = Fbm::default();

    PlaneMapBuilder::new(&noise)
        .set_size(width, height)
        .set_x_bounds(-5., 5.)
        .set_y_bounds(-5., 5.)
        .build()
}

fn gen_mesh(noise: NoiseMap) -> Mesh {
    let (width, height) = noise.size();
    let top_left_x = (width as f32 - 1.) / -2.;
    let top_left_z = (height as f32 - 1.) / -2.;

    let mut vertices = Vec::with_capacity(width * height);
    let mut indices: Vec<u32> = Vec::with_capacity((width - 1) * (height - 1) * 6);

    let mut vertex_index = 0_usize;

    for x in 0..height {
        for y in 0..width {
            vertices.push([
                top_left_x + x as f32,
                noise.get_value(x, y) as f32,
                top_left_z - y as f32,
            ]);

            if x < width - 1 && y < height - 1 {
                indices.push(vertex_index as u32);
                indices.push((vertex_index + width + 1) as u32);
                indices.push((vertex_index + width) as u32);
                indices.push((vertex_index + width + 1) as u32);
                indices.push(vertex_index as u32);
                indices.push((vertex_index + 1) as u32);
            }

            vertex_index += 1;
        }
    }

    let mut mesh = Mesh::new(TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(vertices),
    );
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}

pub fn gen_terrain(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let noise_map = gen_perlin_noise(256, 256);
    let mesh = gen_mesh(noise_map);

    command
        .spawn()
        .insert(Terrain)
        .insert(Transform::from_xyz(0., 0., 0.))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                ..Default::default()
            }),
            ..Default::default()
        });
}
