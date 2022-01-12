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
        .set_x_bounds(-5., 5.)
        .set_y_bounds(-5., 5.)
        .build()
}

fn gen_mesh(noise: NoiseMap) -> Mesh {
    let (width, height) = noise.size();
    let top_left_x = (width as f32 - 1.) / -2.;
    let top_left_z = (height as f32 - 1.) / 2.;

    let mut vertices = Vec::with_capacity(width * height);
    let mut indices = Vec::with_capacity((width - 1) * (height - 1) * 6);
    let mut uvs = Vec::with_capacity(width * height);
    let mut normals = vec![[0., 0., 0.]; width * height];

    let mut vertex_index = 0_usize;

    for y in 0..height {
        for x in 0..width {
            let possition = [
                top_left_x + x as f32,
                (noise.get_value(x, y) * 50.5) as f32,
                top_left_z - y as f32,
            ];

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

    let triangle_count = vertices.len() / 3;
    let mut index = 0;

    'calc_normals: loop {
        if index >= triangle_count {
            break 'calc_normals;
        }

        let normal_index = index * 3;

        let point_a = Vec3::from(vertices[indices[normal_index] as usize]);
        let point_b = Vec3::from(vertices[indices[normal_index + 1] as usize]);
        let point_c = Vec3::from(vertices[indices[normal_index + 2] as usize]);

        let side_ab = point_b - point_a;
        let side_ac = point_c - point_a;

        let normalized = side_ab.cross(side_ac).normalize();

        let normal_a = Vec3::from(normals[normal_index]) + normalized;
        let normal_b = Vec3::from(normals[normal_index + 1]) + normalized;
        let normal_c = Vec3::from(normals[normal_index + 2]) + normalized;

        normals[normal_index] = normal_a.normalize().into();
        normals[normal_index + 1] = normal_b.normalize().into();
        normals[normal_index + 2] = normal_c.normalize().into();

        index += 1;
    }

    // println!("vertices = len: {} {:?}\n", vertices.len(), vertices);
    // println!("normals = len: {} {:?}\n", normals.len(), normals);
    // println!("uvs = len: {} {:?}\n", uvs.len(), uvs);

    let mut mesh = Mesh::new(TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(vertices),
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::Float32x3(normals),
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
    let noise_map = gen_perlin_noise(2040, 2048);
    let mesh = gen_mesh(noise_map);

    commands
        .spawn()
        .insert(Terrain)
        .insert(Transform::from_xyz(0., 0., 0.))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.9, 0.5, 0.3),
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
            ..Default::default()
        });

    let theta = std::f32::consts::FRAC_PI_4;
    let light_transform = Mat4::from_euler(EulerRot::ZYX, 0.0, std::f32::consts::FRAC_PI_2, -theta);
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100000.0,
            shadow_projection: OrthographicProjection {
                left: -0.35,
                right: 500.35,
                bottom: -0.1,
                top: 5.0,
                near: -5.0,
                far: 5.0,
                ..Default::default()
            },
            shadow_depth_bias: 0.9,
            shadow_normal_bias: 2.,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_matrix(light_transform),
        ..Default::default()
    });
}
