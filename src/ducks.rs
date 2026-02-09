use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use rand::*; // 0.8.5

use crate::player::PlayerDuck;

pub fn player_plugin(app: &mut App) {
    app.add_systems(Startup, add_ducks);

    app.add_systems(Update, randomize_targets_on_r);

    app.add_systems(
        FixedUpdate,
        (
            damp_velocity,
            accelerate_ducks,
            control_ducks_with_target_position,
            propagate_duck_physics,
        )
            .chain(),
    );

    app.add_observer(on_add_duck);
}

#[derive(Component, Default, Debug)]
pub struct Duck {
    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub angular_acceleration: f32,
    pub angular_velocity: f32,
}

#[derive(Component, Event, Debug)]
struct AddDuck {
    transform: Transform,
    is_player: bool,
}

#[derive(Component, Debug, Default)]
pub struct TargetPosition {
    pub pos: Vec3,
}

impl TargetPosition {
    fn new() -> Self {
        let x = rand::rng().random_range(-100.0..100.0);
        let z = rand::rng().random_range(-100.0..100.0);
        Self {
            pos: Vec3::new(x, 0.0, z),
        }
    }
}

fn add_ducks(mut commands: Commands) {
    commands.trigger(AddDuck {
        transform: Transform::default(),
        is_player: true,
    });

    for _ in 0..100 {
        let r = rand::rng().random_range(2.0..100.0);
        let a = rand::rng().random_range(0.0..std::f32::consts::PI * 2.0);

        let x = r * a.cos();
        let z = r * a.sin();

        let angle = rand::rng().random_range(0.0..std::f32::consts::PI * 2.0);

        let scale = rand::rng().random_range(0.3..1.0);

        let transform = Transform::from_xyz(x, 0.0, z)
            .with_rotation(Quat::from_rotation_y(angle))
            .with_scale(Vec3::splat(scale));

        commands.trigger(AddDuck {
            transform,
            is_player: false,
        });
    }
}

fn on_add_duck(
    event: On<AddDuck>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let body = meshes.add(Capsule3d::new(0.5, 2.0));
    let head = meshes.add(Capsule3d::new(0.3, 1.0));
    let bill = meshes.add(Capsule3d::new(0.1, 0.7));
    let eye = meshes.add(Sphere::new(0.05));

    let color = Srgba::gray(rand::rng().random_range(0.2..0.99));

    let material = materials.add(StandardMaterial::from_color(color));
    let bill_material = materials.add(StandardMaterial::from_color(YELLOW_400));
    let eye_material = materials.add(StandardMaterial::from_color(GRAY_950));

    let body_transform =
        Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::PI / 2.0));
    let head_transform = Transform::from_xyz(0.0, 1.0, 1.0);
    let bill_transform = Transform::from_xyz(0.0, 1.1, 1.5)
        .with_rotation(Quat::from_rotation_x(std::f32::consts::PI / 2.0))
        .with_scale(Vec3::ONE.with_x(2.0));

    let eye_distance = 0.5;

    let right_eye_transform = Transform::from_xyz(eye_distance / 2.0, 1.4, 1.2);
    let left_eye_transform = Transform::from_xyz(-eye_distance / 2.0, 1.4, 1.2);

    commands
        .spawn((
            Duck::default(),
            event.transform,
            InheritedVisibility::VISIBLE,
        ))
        .with_child((
            body_transform,
            Mesh3d(body),
            MeshMaterial3d(material.clone()),
        ))
        .with_child((
            head_transform,
            Mesh3d(head),
            MeshMaterial3d(material.clone()),
        ))
        .with_child((bill_transform, Mesh3d(bill), MeshMaterial3d(bill_material)))
        .with_child((
            right_eye_transform,
            Mesh3d(eye.clone()),
            MeshMaterial3d(eye_material.clone()),
        ))
        .with_child((
            left_eye_transform,
            Mesh3d(eye),
            MeshMaterial3d(eye_material),
        ))
        .insert_if(PlayerDuck, || event.is_player)
        .insert_if(TargetPosition::new(), || !event.is_player);
}

fn damp_velocity(ducks: Query<&mut Duck>) {
    for mut duck in ducks {
        duck.velocity *= 0.98;
        duck.angular_velocity *= 0.98;
    }
}

fn accelerate_ducks(ducks: Query<&mut Duck>) {
    let dt = 0.02;
    for mut duck in ducks {
        let dv = duck.acceleration * dt;
        duck.velocity += dv;
        let da = duck.angular_acceleration * dt;
        duck.angular_velocity += da;
    }
}

fn propagate_duck_physics(duck: Query<(&Duck, &mut Transform)>, time: Res<Time<Fixed>>) {
    let dt = time.delta_secs();
    for (duck, mut transform) in duck {
        transform.translation += duck.velocity * dt;
        let delta_angle = duck.angular_velocity * dt;
        transform.rotate_local_y(delta_angle);
    }
}

fn control_ducks_with_target_position(ducks: Query<(&mut Duck, &Transform, &TargetPosition)>) {
    for (mut duck, tf, tp) in ducks {
        let delta_pos = tp.pos - tf.translation;
        let accel = (delta_pos).min(Vec3::splat(10.0));
        duck.acceleration = accel;
    }
}

fn randomize_targets_on_r(keys: Res<ButtonInput<KeyCode>>, targets: Query<&mut TargetPosition>) {
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    for mut tp in targets {
        *tp = TargetPosition::new();
    }
}
