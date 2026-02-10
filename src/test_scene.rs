#![allow(unused)]

use bevy::prelude::*;

fn add_test_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere = meshes.add(Sphere::new(0.4));

    for x in 0..=10 {
        for z in 0..=10 {
            let sphere_material = materials.add(StandardMaterial {
                metallic: x as f32 / 10.0,
                reflectance: z as f32 / 10.0,
                ..default()
            });

            let x = x as f32 - 5.0;
            let z = z as f32 - 5.0;

            let tf = Transform::from_xyz(x, 0.0, z);
            commands.spawn((Mesh3d(sphere.clone()), tf, MeshMaterial3d(sphere_material)));
        }
    }

    let platform = meshes.add(Cuboid::new(1.0, 0.1, 1.0));

    let dark_material = materials.add(StandardMaterial {
        base_color: Srgba::gray(0.2).into(),
        ..Default::default()
    });

    let light_material = materials.add(StandardMaterial {
        base_color: Srgba::gray(0.8).into(),
        ..Default::default()
    });

    for x in -20..=20 {
        for z in -20..=20 {
            let mat = if (x + z) % 2 == 0 {
                dark_material.clone()
            } else {
                light_material.clone()
            };

            let tf = Transform::from_xyz(x as f32, -5.0, z as f32);
            commands.spawn((Mesh3d(platform.clone()), MeshMaterial3d(mat), tf));
        }
    }
}
