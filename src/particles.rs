use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use std::time::Duration;

use crate::math::*;

pub fn particles_plugin(app: &mut App) {
    app.add_systems(Startup, setup_resources);
    app.add_systems(
        FixedUpdate,
        (
            spawn_splashes,
            spawn_ripples,
            accelerate_particles_with_gravity,
            propagate_velocity,
            despawn_particles_under_water,
            grow_ripples,
            despawn_ripples_if_too_big,
        )
            .chain(),
    );

    app.add_systems(Update, draw_ripples);

    app.add_message::<Splash>();
}

#[derive(Message, Debug)]
pub struct Splash {
    pub position: Vec3,
    pub velocity: Vec3,
}

#[derive(Component, Default)]
pub struct RippleEmitter {
    last_emitted: Option<Duration>,
    pub is_on: bool,
}

#[derive(Component, Debug)]
struct Particle;

#[derive(Component, Debug)]
struct SplashParticle;

#[derive(Component, Debug)]
struct RippleParticle(f32);

#[derive(Component, Debug, Default)]
struct Velocity(Vec3);

#[derive(Resource)]
struct ParticleResources {
    splash_mesh: Handle<Mesh>,
    ripple_mesh: Handle<Mesh>,
    splash_material: Handle<StandardMaterial>,
    ripple_material: Handle<StandardMaterial>,
}

fn setup_resources(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let splash_mesh = meshes.add(Sphere::new(1.0));
    let splash_material = materials.add(StandardMaterial::from_color(BLUE_500));

    let ripple_mesh = meshes.add(Cylinder::new(1.0, 1.0));
    let ripple_material = materials.add(StandardMaterial::from_color(BLUE_500.with_alpha(0.1)));

    let res = ParticleResources {
        splash_mesh,
        ripple_mesh,
        splash_material,
        ripple_material,
    };

    commands.insert_resource(res);
}

fn spawn_splashes(
    mut commands: Commands,
    resources: Res<ParticleResources>,
    mut messages: MessageReader<Splash>,
) {
    for msg in messages.read() {
        let size = random_range(0.05..=0.25);

        let tf = Transform::from_translation(msg.position).with_scale(Vec3::splat(size));
        commands.spawn((
            Mesh3d(resources.splash_mesh.clone()),
            MeshMaterial3d(resources.splash_material.clone()),
            tf,
            Velocity(msg.velocity),
            SplashParticle,
            Particle,
        ));
    }
}

fn spawn_ripples(
    mut commands: Commands,
    emitters: Query<(&Transform, &mut RippleEmitter)>,
    resources: Res<ParticleResources>,
    time: Res<Time<Fixed>>,
) {
    let now = time.elapsed();

    for (tf, mut emitter) in emitters {
        if !emitter.is_on {
            continue;
        }

        let delta = emitter
            .last_emitted
            .map(|t| now - t)
            .unwrap_or(Duration::from_secs(4));

        if delta < Duration::from_millis(400) {
            continue;
        }

        let pos = Vec3::new(tf.translation.x, 0.0, tf.translation.z);
        let scale = Vec3::new(0.01, 1.0, 0.01);
        let tf = Transform::from_translation(pos).with_scale(scale);
        commands.spawn((
            Mesh3d(resources.ripple_mesh.clone()),
            MeshMaterial3d(resources.ripple_material.clone()),
            tf,
            RippleParticle(0.0),
            Particle,
        ));

        emitter.last_emitted = Some(now);
    }
}

const GRAVITY: f32 = -9.81;

fn accelerate_particles_with_gravity(particles: Query<&mut Velocity, With<SplashParticle>>) {
    let dt = 0.02;
    for mut vel in particles {
        vel.0.y += GRAVITY * dt;
    }
}

fn propagate_velocity(particles: Query<(&mut Transform, &Velocity), With<SplashParticle>>) {
    let dt = 0.02;
    for (mut tf, vel) in particles {
        tf.translation += vel.0 * dt;
    }
}

fn despawn_particles_under_water(
    mut commands: Commands,
    particles: Query<(Entity, &Transform), With<SplashParticle>>,
) {
    for (e, tf) in particles {
        if tf.translation.y < -10.0 {
            commands.entity(e).despawn();
        }
    }
}

fn grow_ripples(particles: Query<&mut RippleParticle>) {
    for mut particle in particles {
        particle.0 += 0.004;
    }
}

fn despawn_ripples_if_too_big(mut commands: Commands, particles: Query<(Entity, &RippleParticle)>) {
    for (e, particle) in particles {
        if particle.0 >= 1.0 {
            commands.entity(e).despawn();
        }
    }
}

fn draw_ripples(mut painter: ShapePainter, particles: Query<(&Transform, &RippleParticle)>) {
    painter.reset();
    painter.hollow = true;

    let rot = Quat::from_rotation_x(std::f32::consts::PI / 2.0);
    painter.set_rotation(rot);

    for (tf, particle) in particles {
        let alpha = 0.2.lerp(0.0, particle.0);
        let t = 0.03.lerp(3.0, particle.0);
        let r = 0.01.lerp(20.0, particle.0);

        painter.thickness = t;
        painter.set_color(BLUE_500.with_alpha(alpha));
        painter.set_translation(tf.translation.with_y(0.1));
        painter.circle(r);
    }
}
