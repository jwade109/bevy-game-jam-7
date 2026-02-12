use std::num::NonZero;

use bevy::prelude::*;
use bevy_rich_text3d::{Text3d, Text3dStyling, TextAtlas};

use crate::math::random_vec;

pub fn text_bubble_plugin(app: &mut App) {
    app.add_systems(Update, point_text_2d_towards_camera);
    app.add_observer(on_spawn_random_bubble);
}

#[derive(Event, Debug)]
pub struct Speak {
    pub entity: Entity,
    pub text: String,
}

#[derive(Component)]
struct TextBubble;

fn on_spawn_random_bubble(
    event: On<Speak>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Duck {} says: {}", event.entity, event.text);

    let p = random_vec(2.0, 15.0);
    let tf = Transform::from_xyz(p.x, 4.0, p.y);

    let mesh = meshes.add(Circle::new(1.0));
    let mat = materials.add(StandardMaterial::from_color(Srgba::gray(0.2)));

    let root = commands
        .spawn((
            TextBubble,
            tf,
            Mesh3d(mesh),
            MeshMaterial3d(mat),
            InheritedVisibility::VISIBLE,
        ))
        .id();

    let e = commands
        .spawn((
            Text3d::new(event.text.clone()),
            Text3dStyling {
                size: 80.,
                stroke: NonZero::new(10),
                color: Srgba::new(1., 1., 1., 1.),
                stroke_color: Srgba::BLACK,
                world_scale: Some(Vec2::splat(0.15)),
                layer_offset: 0.001,
                ..Default::default()
            },
            Transform::from_xyz(0.0, 0.0, 0.01),
            Mesh3d::default(),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(TextAtlas::DEFAULT_IMAGE.clone()),
                alpha_mode: AlphaMode::Blend,
                emissive: Srgba::WHITE.with_alpha(0.4).into(),
                ..Default::default()
            })),
            InheritedVisibility::VISIBLE,
        ))
        .id();

    commands.entity(root).add_child(e);
}

fn point_text_2d_towards_camera(
    mut gizmos: Gizmos,
    text: Query<&mut Transform, With<TextBubble>>,
    camera: Single<&Transform, (With<Camera3d>, Without<TextBubble>)>,
) {
    for mut tf in text {
        let dir = (tf.translation - camera.translation).with_y(0.0);
        tf.look_to(dir, Vec3::Y);
        gizmos.axes(tf.with_scale(Vec3::ONE), 0.3);
    }
}
