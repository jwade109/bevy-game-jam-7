use bevy::asset::AssetMetaCheck;
use bevy::color::palettes::css::*;
use bevy::prelude::*;

mod lake;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        // .add_plugins(camera_plugin)
        .add_plugins(player::player_plugin)
        // .add_plugins(draw_transforms_plugin)
        .add_plugins(lake::lake_plugin)
        .add_systems(Startup, setup)
        .run();
}

fn camera_plugin(app: &mut App) {
    app.add_systems(Update, move_camera);
}

fn draw_transforms_plugin(app: &mut App) {
    app.add_systems(Update, draw_all_transforms);
}

fn move_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
) {
    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        direction += camera.forward().as_vec3()
    }
    if keys.pressed(KeyCode::KeyA) {
        direction += camera.left().as_vec3()
    }
    if keys.pressed(KeyCode::KeyS) {
        direction += camera.back().as_vec3()
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += camera.right().as_vec3()
    }
    if keys.pressed(KeyCode::ControlLeft) {
        direction += camera.down().as_vec3()
    }
    if keys.pressed(KeyCode::Space) {
        direction += camera.up().as_vec3()
    }

    let speed = 0.1;

    camera.translation += direction.normalize_or_zero() * speed;

    camera.look_at(Vec3::ZERO, Vec3::Y);
}

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

fn draw_all_transforms(mut gizmos: Gizmos, transforms: Query<&GlobalTransform>) {
    for tf in transforms {
        let tf = tf.compute_transform();
        gizmos.axes(tf, 0.5);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(12.0, 20.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 7.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
