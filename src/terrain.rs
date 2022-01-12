use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology::TriangleList,
    },
};
use noise::{
    utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder},
    Perlin,
};

#[derive(Component)]
pub struct Terrain;

fn gen_perlin_noise(width: usize, height: usize) -> NoiseMap {
    let noise = Perlin::default();

    PlaneMapBuilder::new(&noise)
        .set_size(width, height)
        .set_x_bounds(0., 5.)
        .set_y_bounds(0., 5.)
        .build()
}

fn gen_mesh(noise: NoiseMap) -> Mesh {
    let (width, height) = noise.size();
    let top_left_x = (width as f32 - 1.) / -2.;
    let top_left_z = (height as f32 - 1.) / 2.;

    let mut vertices = Vec::with_capacity(width * height);
    let mut indices = Vec::with_capacity((width - 1) * (height - 1) * 6);
    let mut uvs = Vec::with_capacity(width * height);

    let mut vertex_index = 0_usize;

    for y in 0..height {
        for x in 0..width {
            let possition = [
                top_left_x + x as f32,
                noise.get_value(x, y) as f32,
                top_left_z - y as f32,
            ];

            println!("{}: {:?}", vertex_index, &possition);

            vertices.push(possition);

            if x < width - 1 && y < height - 1 {
                indices.push(vertex_index as u32);
                indices.push((vertex_index + width + 1) as u32);
                indices.push((vertex_index + width) as u32);

                indices.push((vertex_index + width + 1) as u32);
                indices.push(vertex_index as u32);
                indices.push((vertex_index + 1) as u32);
            }

            uvs.push([
                x as f32 / (width - 1) as f32,
                y as f32 / (height - 1) as f32,
            ]);

            vertex_index += 1;
        }
    }

    println!("{:?}", indices);
    println!("{:?}", uvs);

    let mut mesh = Mesh::new(TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(vertices),
    );
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float32x2(uvs));
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}

pub fn gen_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let noise_map = gen_perlin_noise(25, 25);
    let mesh = gen_mesh(noise_map);

    commands
        .spawn()
        .insert(Terrain)
        .insert(Transform::from_xyz(0., 0., 0.))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.9, 0.5, 0.3).into()),
            ..Default::default()
        });

    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 5. })),
    //         material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //         ..Default::default()
    //     })
    //     .insert(Transform::from_xyz(0., 5., 0.));
}
