use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use rand::*; // 0.8.5

use bevy::{prelude::*, time::Stopwatch};

use crate::math::random_chance;
use crate::particles::{RippleEmitter, Splash};
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
            move_duck_heads,
            update_head_turning_transform,
        )
            .chain(),
    );

    app.add_systems(
        FixedUpdate,
        (spawn_particles_if_kicking, spawn_ripples_if_kicking),
    );

    app.add_observer(on_add_duck);
}

#[derive(Component, Default, Debug)]
pub struct Duck {
    pub is_kicking: bool,
    pub velocity: Vec3,
    pub angular_acceleration: f32,
    pub angular_velocity: f32,
    pub target_head_angle: f32,
    pub actual_head_angle: f32,
}

impl Duck {
    pub fn body_fixed_acceleration(&self) -> Vec3 {
        Vec3::Z * self.is_kicking as u8 as f32 * 10.0
    }
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

#[derive(Component)]
pub struct HeadRoot(Entity);

fn update_head_turning_transform(
    ducks: Query<&Duck>,
    head_transforms: Query<(&mut Transform, &HeadRoot)>,
) -> Result {
    for (mut transform, root) in head_transforms {
        let duck = ducks.get(root.0)?;
        transform.rotation = Quat::from_rotation_y(duck.actual_head_angle);
    }
    Ok(())
}

fn on_add_duck(
    event: On<AddDuck>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let body = meshes.add(Capsule3d::new(0.5, 2.0));
    let head = meshes.add(Capsule3d::new(0.3, 1.0));
    let eye = meshes.add(Sphere::new(0.05));

    let color = Srgba::gray(rand::rng().random_range(0.2..0.99));

    let material = materials.add(StandardMaterial::from_color(color));
    let bill_material = materials.add(StandardMaterial::from_color(YELLOW_400));
    let eye_material = materials.add(StandardMaterial::from_color(GRAY_950));

    let body_transform =
        Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::PI / 2.0));
    let head_transform = Transform::from_xyz(0.0, 1.0, 1.0);

    let bill_length = 0.7;
    let bill_width = 2.0;
    let bill_height_above_head_center = 0.4;

    let bill = meshes.add(Capsule3d::new(0.1, bill_length));
    let bill_transform = Transform::from_xyz(0.0, bill_height_above_head_center, bill_length / 2.3)
        .with_rotation(Quat::from_rotation_x(std::f32::consts::PI / 2.0))
        .with_scale(Vec3::ONE.with_x(bill_width));

    let eye_distance = 0.5;

    let right_eye_transform = Transform::from_xyz(eye_distance / 2.0, 0.6, 0.2);
    let left_eye_transform = Transform::from_xyz(-eye_distance / 2.0, 0.6, 0.2);

    let root = commands
        .spawn((
            Duck {
                actual_head_angle: rand::rng().random_range(-0.3..=0.3),
                is_kicking: random_chance(0.2),
                ..default()
            },
            event.transform,
            RippleEmitter::default(),
            InheritedVisibility::VISIBLE,
        ))
        .with_child((
            body_transform,
            Mesh3d(body),
            MeshMaterial3d(material.clone()),
        ))
        .insert_if(PlayerDuck, || event.is_player)
        .insert_if(TargetPosition::new(), || !event.is_player)
        .id();

    let head = commands
        .spawn((
            head_transform,
            HeadRoot(root),
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
        .id();

    commands.entity(root).add_child(head);

    // if random_chance(0.8) {

    let idx = random_range(2..=5);
    let name = format!("wek{idx}.ogg");

    commands.entity(root).insert((
        AudioPlayer::new(asset_server.load(name)),
        PlaybackSettings::LOOP.with_spatial(true),
    ));
    // }
}

fn damp_velocity(ducks: Query<&mut Duck>) {
    for mut duck in ducks {
        duck.velocity *= 0.95;
        duck.angular_velocity *= 0.95;
    }
}

fn accelerate_ducks(ducks: Query<(&mut Duck, &Transform)>) {
    let dt = 0.02;
    for (mut duck, tf) in ducks {
        let bfa = duck.body_fixed_acceleration();
        let accel = tf.local_x() * bfa.x + tf.local_y() * bfa.y + tf.local_z() * bfa.z;
        let dv = accel * dt;
        duck.velocity += dv;
        let da = duck.angular_acceleration * dt;
        duck.angular_velocity += da;
    }
}

fn move_duck_heads(ducks: Query<&mut Duck>) {
    let max_rate = 0.06;
    for mut duck in ducks {
        if rand::rng().random_bool(0.01) {
            duck.target_head_angle = rand::rng().random_range(-2.0..=2.0);
        }

        let delta = duck.target_head_angle - duck.actual_head_angle;
        let delta = delta.clamp(-max_rate, max_rate);
        duck.actual_head_angle += delta;
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
        // duck.acceleration = accel;
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

fn spawn_particles_if_kicking(
    mut messages: MessageWriter<Splash>,
    ducks: Query<(&Duck, &Transform)>,
) {
    for (duck, transform) in ducks {
        if duck.is_kicking {
            let vx = rand::rng().random_range(-5.0..=5.0);
            let vy = rand::rng().random_range(2.0..=5.0);
            let vz = rand::rng().random_range(-5.0..=5.0);

            let splash = Splash {
                position: transform.translation,
                velocity: Vec3::new(vx, vy, vz),
            };

            messages.write(splash);
        }
    }
}

fn spawn_ripples_if_kicking(ducks: Query<(&Duck, &mut RippleEmitter)>) {
    for (duck, mut emitter) in ducks {
        emitter.is_on = duck.is_kicking || random_chance(0.001);
    }
}
