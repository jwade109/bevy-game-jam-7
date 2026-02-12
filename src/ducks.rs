use bevy::audio::Volume;
use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use rand::*;

use crate::math::{random_chance, random_vec};
use crate::particles::{RippleEmitter, Splash};
use crate::player::PlayerDuck;
use crate::text_bubble::Speak;

pub fn player_plugin(app: &mut App) {
    app.add_systems(Startup, add_ducks);

    app.add_systems(Update, randomize_targets_on_r);

    app.add_systems(
        FixedUpdate,
        (
            assign_parent_to_parentless_ducks,
            damp_velocity,
            apply_gravity_to_ducks,
            update_ducks_above_sea_level,
            accelerate_ducks,
            update_tracking_force_for_target_seekers,
            update_separation_force,
            randomly_wander,
            update_target_pos_for_ducks_with_parents,
            control_boids,
            propagate_duck_physics,
            move_duck_heads,
            update_head_turning_transform,
            randomly_speak,
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
    pub speed_mod: f32,
    pub velocity: Vec3,
    pub angular_acceleration: f32,
    pub angular_velocity: f32,
    pub target_head_angle: f32,
    pub actual_head_angle: f32,
    pub above_sea_level: f32,
}

impl Duck {
    pub fn is_in_water(&self) -> bool {
        self.above_sea_level < 0.0
    }

    pub fn body_fixed_acceleration(&self) -> Vec3 {
        let buoyancy = (-self.above_sea_level).max(0.0) * 100.0;
        let kicking = if self.is_in_water() {
            self.is_kicking as u8 as f32 * 10.0
        } else {
            0.0
        };
        Vec3::Z * kicking * self.speed_mod + Vec3::Y * buoyancy
    }

    pub fn move_with_force(&mut self, tf: Transform, force: Vec3, radius: f32) {
        if force.length() < radius {
            self.is_kicking = false;
            self.angular_acceleration = 0.0;
            return;
        }

        let right = tf.local_x();
        let forward = tf.local_z();

        let angle = force.angle_between(forward.as_vec3());
        let right_component = force.dot(right.as_vec3());

        let turn = if right_component > 0.0 {
            angle.abs()
        } else {
            -angle.abs()
        };

        self.is_kicking = angle.abs() < std::f32::consts::PI / 2.0;

        self.angular_acceleration = turn * 7.0;
    }
}

#[derive(Component, Event, Debug)]
struct AddDuck {
    transform: Transform,
    is_player: bool,
    is_child: bool,
}

#[derive(Component, Debug, Default)]
pub struct TargetPosition {
    pub pos: Vec3,
}

#[derive(Component)]
pub struct Duckling;

#[derive(Component, Debug)]
pub struct ParentDuck {
    pub duck: Entity,
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

fn assign_parent_to_parentless_ducks(
    mut commands: Commands,
    adults: Query<(Entity, &Transform), (With<Duck>, Without<Duckling>)>,
    parentless: Query<(Entity, &Transform), (With<Duckling>, Without<ParentDuck>)>,
) -> Result {
    for (pl, p) in parentless {
        for (e, q) in adults {
            let dist = p.translation.distance(q.translation);
            if dist < 15.0 {
                commands.entity(pl).insert(ParentDuck { duck: e });
                info!("Assigned duckling {} parent of {}", pl, e);
                break;
            }
        }
    }
    Ok(())
}

const NUM_DUCKS: usize = 20;

fn add_ducks(mut commands: Commands) {
    commands.trigger(AddDuck {
        transform: Transform::default(),
        is_player: true,
        is_child: false,
    });

    for _ in 0..NUM_DUCKS {
        let r = rand::rng().random_range(2.0..100.0);
        let a = rand::rng().random_range(0.0..std::f32::consts::PI * 2.0);

        let x = r * a.cos();
        let z = r * a.sin();

        let angle = rand::rng().random_range(0.0..std::f32::consts::PI * 2.0);

        let is_child = random_chance(0.9);

        let scale = if is_child {
            random_range(0.2..0.3)
        } else {
            random_range(0.6..1.0)
        };

        let transform = Transform::from_xyz(x, 0.0, z)
            .with_rotation(Quat::from_rotation_y(angle))
            .with_scale(Vec3::splat(scale));

        commands.trigger(AddDuck {
            transform,
            is_player: false,
            is_child,
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

    let speed_mod = if event.is_child {
        random_range(1.3..=2.1)
    } else {
        random_range(0.9..=1.1)
    };

    let root = commands
        .spawn((
            Duck {
                actual_head_angle: rand::rng().random_range(-0.3..=0.3),
                is_kicking: random_chance(0.2),
                velocity: Vec3::Y * 3.0,
                speed_mod,
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

    if event.is_child {
        commands.entity(root).insert(Duckling);
    }

    if !event.is_player {
        let idx = if event.is_child { 4 } else { 2 };
        let name = format!("wek{idx}.ogg");
        let speed = random_range(0.95..=1.5);
        commands.entity(root).insert((
            AudioPlayer::new(asset_server.load(name)),
            PlaybackSettings::LOOP
                .with_spatial(true)
                .with_duration(std::time::Duration::from_secs(3))
                .with_speed(speed)
                .with_volume(Volume::Linear(0.3)),
            Boid::default(),
        ));
    }
}

fn update_target_pos_for_ducks_with_parents(
    ducks: Query<(&ParentDuck, &mut TargetPosition)>,
    transforms: Query<&Transform>,
) -> Result {
    for (parent, mut tp) in ducks {
        let tf = transforms.get(parent.duck)?;
        tp.pos = tf.translation;
    }
    Ok(())
}

fn damp_velocity(ducks: Query<&mut Duck>) {
    for mut duck in ducks {
        if duck.above_sea_level <= 0.0 {
            duck.velocity.y *= 0.8;
            duck.velocity.x *= 0.95;
            duck.velocity.z *= 0.95;
        }
        duck.angular_velocity *= 0.95;
    }
}

const GRAVITY: f32 = -9.81;

fn apply_gravity_to_ducks(ducks: Query<(&mut Duck, &mut Transform)>) {
    let dt = 0.02;
    for (mut duck, tf) in ducks {
        if tf.translation.y > 0.0 {
            duck.velocity.y += GRAVITY * dt;
        }
    }
}

fn update_ducks_above_sea_level(ducks: Query<(&mut Duck, &Transform)>) {
    for (mut duck, tf) in ducks {
        duck.above_sea_level = tf.translation.y;
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

/// Tracks all the behaviors forces acting on this duck
#[derive(Component, Default, Debug)]
pub struct Boid {
    pub separation: Vec3,
    pub seek_target: Vec3,
}

impl Boid {
    pub fn total_force(&self) -> Vec3 {
        self.seek_target + self.separation
    }
}

fn update_tracking_force_for_target_seekers(
    boids: Query<(&mut Boid, &Transform, &TargetPosition)>,
) {
    for (mut boid, tf, tp) in boids {
        let delta = tp.pos - tf.translation;
        boid.seek_target = delta.normalize_or_zero() * delta.length().clamp(0.0, 10.0);
    }
}

fn update_separation_force(
    boids: Query<(Entity, &mut Boid, &Transform, Option<&Duckling>)>,
    ducks: Query<(Entity, &Transform, Option<&Duckling>), With<Duck>>,
) {
    for (e1, mut boid, p, ego) in boids {
        boid.separation = Vec3::ZERO;
        for (e2, q, other) in ducks {
            if e1 == e2 {
                continue;
            }

            let ego_is_adult = ego.is_none();
            let other_is_adult = other.is_none();

            let weight = match (ego_is_adult, other_is_adult) {
                (true, true) => 200.0,
                (true, false) => 0.0,
                (false, true) => 10.0,
                (false, false) => 1.0,
            };

            let delta = p.translation - q.translation;

            let force = delta.normalize_or_zero() * 1.0 / (1.0 + delta.length().powi(2));

            boid.separation += force * weight;
        }
    }
}

fn randomly_wander(targets: Query<&mut TargetPosition>) {
    for mut target in targets {
        if random_chance(0.001) {
            let delta = random_vec(0.1, 4.0);
            target.pos.x += delta.x;
            target.pos.z += delta.y;
        } else if random_chance(0.00003) {
            let pos = random_vec(0.0, 200.0);
            target.pos.x = pos.x;
            target.pos.z = pos.y;
        }
    }
}

fn control_boids(ducks: Query<(&mut Duck, &Transform, &Boid)>) {
    for (mut duck, tf, boid) in ducks {
        let force = boid.total_force();
        duck.move_with_force(*tf, force, 1.5);
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
        if duck.is_kicking && duck.is_in_water() {
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
        if !duck.is_in_water() {
            continue;
        }
        emitter.is_on = duck.is_kicking || random_chance(0.001);
    }
}

fn randomly_speak(mut commands: Commands, ducks: Query<Entity, With<Duckling>>) {
    for duck in ducks {
        if random_chance(0.001) {
            let msg = "HELLO THERE";
            commands.trigger(Speak {
                entity: duck,
                text: msg.to_string(),
            })
        }
    }
}
