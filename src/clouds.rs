use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use noiz::prelude::*;

use crate::math::random_range;

pub fn clouds_plugin(app: &mut App) {
    app.add_systems(Startup, add_clouds);

    app.add_systems(
        FixedUpdate,
        (update_noise_sample_offset, sample_cloud_size).chain(),
    );
}

const NUM_CLOUDS: usize = 2000;
const CLOUD_RANGE: f32 = 5000.0;
const CLOUD_HEIGHT: f32 = 500.0;
const CLOUD_MAX_RADIUS: f32 = 400.0;
const CLOUD_NOISE_SCALE: f32 = 2000.0;
const CLOUD_SPEED: f32 = 0.0003;

#[derive(Resource, Deref, DerefMut)]
struct NoiseFunc(Noise<MixCellGradients<OrthoGrid, Smoothstep, QuickGradients>>);

#[derive(Resource, Deref, DerefMut)]
struct NoiseOffset(Vec2);

#[derive(Component)]
struct Cloud;

fn add_clouds(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cloud_mesh = meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap());

    let mut cloud_material = StandardMaterial::from_color(GRAY_50);
    cloud_material.emissive = GRAY_50.into();

    let cloud_material = materials.add(cloud_material);

    let noise = Noise::<MixCellGradients<OrthoGrid, Smoothstep, QuickGradients>>::default();

    commands.insert_resource(NoiseFunc(noise));
    commands.insert_resource(NoiseOffset(Vec2::ZERO));

    for _ in 0..NUM_CLOUDS {
        let x = random_range(-1.0..=1.0) * CLOUD_RANGE;
        let z = random_range(-1.0..=1.0) * CLOUD_RANGE;

        // let radius = random_range(10.0..40.0);

        let transform = Transform::from_xyz(x, CLOUD_HEIGHT, z);

        commands.spawn((
            transform,
            Mesh3d(cloud_mesh.clone()),
            Cloud,
            MeshMaterial3d(cloud_material.clone()),
        ));
    }
}

fn update_noise_sample_offset(mut offset: ResMut<NoiseOffset>) {
    offset.0 += Vec2::splat(CLOUD_SPEED);
}

fn sample_cloud_size(
    clouds: Query<&mut Transform, With<Cloud>>,
    noise: Res<NoiseFunc>,
    offset: Res<NoiseOffset>,
) {
    for mut tf in clouds {
        let p = tf.translation.xz() / CLOUD_NOISE_SCALE + offset.0;
        let t: f32 = noise.sample(p); // [-1, 1]
        // let t = (t + 1.0) / 2.0; // [0, 1]
        let radius = t.clamp(0.0, 1.0) * CLOUD_MAX_RADIUS;
        tf.scale = Vec3::splat(radius);
    }
}
