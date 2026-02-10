#![allow(unused)]

use bevy::prelude::*;

fn camera_plugin(app: &mut App) {
    app.add_systems(Update, move_camera);
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
