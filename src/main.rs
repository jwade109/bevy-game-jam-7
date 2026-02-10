use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

mod debug;
mod detached_camera;
mod ducks;
mod lake;
mod math;
mod particles;
mod player;
mod test_scene;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins(ShapePlugin::default())
        // .add_plugins(camera_plugin)
        .add_plugins(player::player_plugin)
        .add_plugins(ducks::player_plugin)
        .add_plugins(debug::debug_plugin)
        .add_plugins(lake::lake_plugin)
        .add_plugins(particles::particles_plugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(12.0, 20.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Camera3d::default(),
        SpatialListener::new(1.0),
        Transform::from_xyz(12.0, 25.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
