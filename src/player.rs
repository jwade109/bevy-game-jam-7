use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::ducks::{Duck, DuckJump};

pub fn player_plugin(app: &mut App) {
    app.insert_resource(CameraScale {
        target: 0.0,
        actual: 1.0,
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
    let c1 = Vec3::new(2.0, 2.0, -5.0);
    let c2 = Vec3::new(0.0, 30.0, -15.0);

    let f1 = Vec3::new(0.0, 0.0, 10.0);
    let f2 = Vec3::new(0.0, 0.0, 15.0);

    let c = c1.lerp(c2, scale);
    let f = f1.lerp(f2, scale);

    let mut camera = player * Transform::from_translation(c);
    let focus_transform = Transform::from_translation(f);
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
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    ducks: Query<(Entity, &mut Duck, &Transform), With<PlayerDuck>>,
) {
    for (e, mut duck, transform) in ducks {
        let mut velocity = Vec3::ZERO;
        let mut angular_velocity = 0.0;

        // jumping
        if keys.just_pressed(KeyCode::Space) {
            commands.write_message(DuckJump { duck: e });
        }

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

        duck.is_kicking = keys.pressed(KeyCode::KeyW);
        duck.is_boosting = keys.pressed(KeyCode::ShiftLeft);

        duck.angular_acceleration = angular_velocity * 2.0;
    }
}
