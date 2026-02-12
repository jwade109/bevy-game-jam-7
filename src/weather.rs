use bevy::color::palettes::tailwind::*;
use bevy::{audio::Volume, prelude::*};

use crate::{
    clouds::{SetCloudColor, SetWindSpeed},
    math::{random_chance, random_range},
};

pub fn weather_plugin(app: &mut App) {
    // app.add_systems(Startup, add_rain_sounds);
    app.add_systems(Startup, (add_sunlight, set_sky_color));

    app.add_systems(Update, (spawn_lightning_on_l, toggle_weather_on_m));
    app.add_systems(
        FixedUpdate,
        (
            update_lightning,
            randomly_spawn_lightning.run_if(in_state(Weather::Thunderstorm)),
        ),
    );

    app.add_systems(
        OnEnter(Weather::Clear),
        (on_clear_weather, remove_rain_sounds, brighten_sun),
    );
    app.add_systems(
        OnEnter(Weather::Thunderstorm),
        (on_thunderstorm, add_rain_sounds, darken_sun),
    );

    app.add_observer(on_lightning);

    app.insert_state(Weather::Clear);
}

#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Weather {
    Clear,
    Thunderstorm,
}

fn on_clear_weather(mut commands: Commands, mut color: ResMut<ClearColor>) {
    info!("Clear weather!");
    commands.trigger(SetWindSpeed(0.3));
    commands.trigger(SetCloudColor(Srgba::gray(0.95).into()));

    color.0 = BLUE_300.into();
}

fn on_thunderstorm(mut commands: Commands, mut color: ResMut<ClearColor>) {
    info!("Thunderstorm!");
    commands.trigger(SetWindSpeed(7.0));
    commands.trigger(SetCloudColor(Srgba::gray(0.04).into()));

    color.0 = GRAY_700.into();
}

fn brighten_sun(mut sun: Single<&mut DirectionalLight, With<Sun>>) {
    sun.illuminance = light_consts::lux::OVERCAST_DAY;
}

fn darken_sun(mut sun: Single<&mut DirectionalLight, With<Sun>>) {
    sun.illuminance = light_consts::lux::FULL_MOON_NIGHT;
}

fn randomly_spawn_lightning(mut commands: Commands) {
    if random_chance(0.001) {
        commands.trigger(LightningEvent);
    }
}

fn toggle_weather_on_m(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<Weather>>,
    mut next: ResMut<NextState<Weather>>,
) {
    if keys.just_pressed(KeyCode::KeyM) {
        match **state {
            Weather::Clear => next.set(Weather::Thunderstorm),
            Weather::Thunderstorm => next.set(Weather::Clear),
        }
    }
}

#[derive(Event)]
struct LightningEvent;

#[derive(Component)]
struct Lightning;

fn set_sky_color(mut color: ResMut<ClearColor>) {
    color.0 = BLUE_300.into();
}

#[derive(Component)]
struct Sun;

fn add_sunlight(mut commands: Commands) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(12.0, 20.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
        Sun,
    ));
}

#[derive(Component)]
struct RainSound;

fn add_rain_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("rain.ogg")),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.3)),
        RainSound,
    ));
}

fn remove_rain_sounds(mut commands: Commands, sounds: Query<Entity, With<RainSound>>) {
    for e in sounds {
        commands.entity(e).despawn();
    }
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
