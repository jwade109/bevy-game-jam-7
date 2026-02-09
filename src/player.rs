use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::ducks::Duck;

pub fn player_plugin(app: &mut App) {
    app.insert_resource(CameraScale {
        target: 0.0,
        actual: 0.5,
    });

    app.add_systems(
        Update,
        (
            handle_player_keys,
            update_camera_scale,
            make_camera_follow_player,
        )
            .chain(),
    );
}

#[derive(Resource)]
struct CameraScale {
    target: f32,
    actual: f32,
}

#[derive(Component)]
pub struct PlayerDuck;

fn camera_transform(player: Transform, scale: f32) -> Transform {
    let cx = 2.0.lerp(0.0, scale);
    let cy = 2.0.lerp(80.0, scale);
    let cz = -5.0.lerp(-1.0, scale);

    let mut camera = player * Transform::from_xyz(cx, cy, cz);

    let fx = 0.0;
    let fy = 0.0;
    let fz = 10.0.lerp(2.0, scale);

    let focus_transform = Transform::from_xyz(fx, fy, fz);

    let focus = player * focus_transform;

    camera.look_at(focus.translation, Vec3::Y);

    camera
}

fn make_camera_follow_player(
    player: Single<&Transform, With<PlayerDuck>>,
    mut camera: Single<&mut Transform, (With<Camera3d>, Without<PlayerDuck>)>,
    scale: Res<CameraScale>,
) {
    **camera = camera_transform(**player, scale.actual);
}

fn update_camera_scale(mut events: MessageReader<MouseWheel>, mut scale: ResMut<CameraScale>) {
    let mut delta = 0.0;

    let rate = 0.06;

    for event in events.read() {
        if event.y < 0.0 {
            delta += rate;
        } else {
            delta -= rate;
        }
    }

    scale.target += delta;

    scale.target = scale.target.clamp(0.0, 1.0);

    scale.actual += (scale.target - scale.actual) * 0.05;
}

fn handle_player_keys(
    keys: Res<ButtonInput<KeyCode>>,
    ducks: Query<(&mut Duck, &Transform), With<PlayerDuck>>,
) {
    for (mut duck, transform) in ducks {
        let mut velocity = Vec3::ZERO;
        let mut angular_velocity = 0.0;

        // forward
        if keys.pressed(KeyCode::KeyW) {
            velocity += transform.local_z().as_vec3()
        }

        // left
        if keys.pressed(KeyCode::KeyA) {
            angular_velocity += 1.0;
        }

        // backward
        if keys.pressed(KeyCode::KeyS) {
            velocity -= transform.local_z().as_vec3()
        }

        // right
        if keys.pressed(KeyCode::KeyD) {
            angular_velocity -= 1.0;
        }

        duck.acceleration = velocity * 6.0;
        duck.angular_acceleration = angular_velocity * 2.0;
    }
}
