use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_rich_text3d::{LoadFonts, Text3dPlugin};
use bevy_vector_shapes::prelude::*;

mod clouds;
mod debug;
mod despawn_after;
mod detached_camera;
mod ducks;
mod lake;
mod math;
mod particles;
mod player;
mod test_scene;
mod text_bubble;
mod weather;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        // Wasm builds will check for meta files (that don't exist) if this isn't set.
        // This causes errors and even panics in web builds on itch.
        // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
        meta_check: AssetMetaCheck::Never,
        ..default()
    }));

    app.add_plugins(Text3dPlugin {
        default_atlas_dimension: (2048, 2048),
        load_system_fonts: true,
        ..Default::default()
    });

    app.insert_resource(LoadFonts {
        font_paths: vec!["assets/SNPro-Regular.ttf".to_owned()],
        font_directories: vec!["assets/fonts".to_owned()],
        ..Default::default()
    });

    app.add_plugins(ShapePlugin::default())
        // .add_plugins(camera_plugin)
        .add_plugins(player::player_plugin)
        .add_plugins(ducks::player_plugin)
        .add_plugins(debug::debug_plugin)
        .add_plugins(lake::lake_plugin)
        .add_plugins(particles::particles_plugin)
        .add_plugins(clouds::clouds_plugin)
        .add_plugins(weather::weather_plugin)
        .add_plugins(text_bubble::text_bubble_plugin)
        .add_plugins(despawn_after::despawn_after_plugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: std::f32::consts::PI / 2.0,
            ..default()
        }),
        SpatialListener::new(1.0),
        Transform::from_xyz(12.0, 25.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
