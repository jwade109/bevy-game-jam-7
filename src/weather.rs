use bevy::{audio::Volume, color::palettes::tailwind::BLUE_300, prelude::*};

use crate::math::{random_chance, random_range};

pub fn weather_plugin(app: &mut App) {
    // app.add_systems(Startup, add_rain_sounds);
    app.add_systems(Startup, (add_sunlight, set_sky_color));

    app.add_systems(Update, spawn_lightning_on_l);
    app.add_systems(FixedUpdate, update_lightning);

    app.add_observer(on_lightning);
}

#[derive(Event)]
struct LightningEvent;

#[derive(Component)]
struct Lightning;

fn set_sky_color(mut color: ResMut<ClearColor>) {
    color.0 = BLUE_300.into();
}

fn add_sunlight(mut commands: Commands) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(12.0, 20.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn add_rain_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("rain.ogg")),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.3)),
    ));
}

fn on_lightning(
    _event: On<LightningEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let x = random_range(-100.0..100.0);
    let z = random_range(-100.0..100.0);
    let y = random_range(20.0..100.0);

    info!("Lightning: {} {} {}", x, y, z);

    let tf = Transform::from_xyz(x, y, z);

    commands.spawn((
        PointLight {
            intensity: 10000000000000.0,
            range: 1000000.0,
            shadows_enabled: true,
            ..default()
        },
        tf,
        Lightning,
    ));

    commands.spawn((
        AudioPlayer::new(asset_server.load("thunder1.ogg")),
        PlaybackSettings::DESPAWN,
    ));
}

fn update_lightning(
    mut commands: Commands,
    lights: Query<(Entity, &mut PointLight), With<Lightning>>,
) {
    for (e, mut light) in lights {
        light.intensity *= 0.9;
        light.range *= 0.9;

        if light.intensity < 5.0 {
            commands.entity(e).despawn();
        }
    }
}

fn spawn_lightning_on_l(mut commands: Commands, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyL) {
        commands.trigger(LightningEvent);
    }
}
