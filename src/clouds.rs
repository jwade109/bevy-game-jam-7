use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use noiz::prelude::*;

use crate::math::random_range;

pub fn clouds_plugin(app: &mut App) {
    app.add_systems(Startup, add_clouds);

    app.add_systems(
        FixedUpdate,
        (
            update_noise_sample_offset,
            sample_cloud_size,
            update_lpf_cloud_speed,
            update_cloud_material,
        )
            .chain(),
    );

    app.add_observer(on_set_wind_speed);

    app.add_observer(on_set_cloud_color);

    let noise = Noise::<MixCellGradients<OrthoGrid, Smoothstep, QuickGradients>>::default();

    app.insert_resource(NoiseFunc(noise));
    app.insert_resource(NoiseOffset(Vec2::ZERO));
    app.insert_resource(CloudSpeed {
        target: 3.0,
        actual: 120.0,
    });
    app.insert_resource(CloudColor(Srgba::RED.into()));
}

const NUM_CLOUDS: usize = 3000;
const CLOUD_RANGE: f32 = 10000.0;
const CLOUD_HEIGHT: f32 = 900.0;
const CLOUD_MAX_RADIUS: f32 = 800.0;
const CLOUD_NOISE_SCALE: f32 = 2000.0;

#[derive(Resource, Deref, DerefMut)]
struct NoiseFunc(Noise<MixCellGradients<OrthoGrid, Smoothstep, QuickGradients>>);

#[derive(Resource, Deref, DerefMut)]
struct NoiseOffset(Vec2);

#[derive(Resource, Debug)]
struct CloudSpeed {
    target: f32,
    actual: f32,
}

#[derive(Resource, Debug)]
struct CloudColor(Color);

#[derive(Resource, Debug)]
struct CloudMaterial(Handle<StandardMaterial>);

fn update_cloud_material(
    handle: Res<CloudMaterial>,
    color: Res<CloudColor>,
    mut mat: ResMut<Assets<StandardMaterial>>,
) {
    let Some(material) = mat.get_mut(&handle.0) else {
        return;
    };

    material.base_color = color.0;
}

#[derive(Component)]
struct Cloud;

#[derive(Event, Debug)]
pub struct SetWindSpeed(pub f32);

fn on_set_wind_speed(event: On<SetWindSpeed>, mut speed: ResMut<CloudSpeed>) {
    info!("Set wind speed: {:?}", event);
    speed.target = event.0;
}

#[derive(Event, Debug)]
pub struct SetCloudColor(pub Color);

fn on_set_cloud_color(event: On<SetCloudColor>, mut color: ResMut<CloudColor>) {
    info!("Set cloud color: {:?}", event);
    color.0 = event.0;
}

fn add_clouds(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cloud_mesh = meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap());

    let cloud_material = materials.add(StandardMaterial::from_color(GRAY_600));

    commands.insert_resource(CloudMaterial(cloud_material.clone()));

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

fn update_noise_sample_offset(mut offset: ResMut<NoiseOffset>, speed: Res<CloudSpeed>) {
    offset.0 += Vec2::splat(speed.actual / 1000.0);
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
        tf.scale = Vec3::new(radius, radius / 2.0, radius);
    }
}

fn update_lpf_cloud_speed(mut speed: ResMut<CloudSpeed>) {
    speed.actual += (speed.target - speed.actual) * 0.03;
}
