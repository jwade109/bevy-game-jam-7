use std::num::NonZero;

use bevy::prelude::*;
use bevy_rich_text3d::{Text3d, Text3dStyling, TextAtlas};

use crate::{
    despawn_after::DespawnAfter,
    math::{random_range, random_vec},
};

pub fn text_bubble_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (point_text_2d_towards_camera, sync_transforms_to_parent),
    );

    app.add_observer(on_spawn_random_bubble);
}

#[derive(Event, Debug, Clone)]
pub struct Quack {
    pub entity: Entity,
    pub text: String,
}

#[derive(Component)]
struct TextBubble {
    parent: Entity,
    offset: Vec3,
}

fn on_spawn_random_bubble(
    event: On<Quack>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let p = random_vec(2.0, 15.0);
    let tf = Transform::from_xyz(p.x, 4.0, p.y);

    let xz = random_vec(0.03, 0.3);
    let y = random_range(1.2..=2.0);

    let root = commands
        .spawn((
            DespawnAfter::new(std::time::Duration::from_secs(2)),
            TextBubble {
                parent: event.entity,
                offset: Vec3::new(xz.x, y, xz.y),
            },
            tf,
            InheritedVisibility::VISIBLE,
        ))
        .id();

    let e = commands
        .spawn((
            Text3d::new(event.text.clone()),
            Text3dStyling {
                size: 80.,
                color: Srgba::new(1., 1., 1., 1.),
                world_scale: Some(Vec2::splat(0.23)),
                layer_offset: 0.001,
                font: "SNPro-Regular".into(),
                ..Default::default()
            },
            Transform::from_xyz(0.0, 0.0, 0.01),
            Mesh3d::default(),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(TextAtlas::DEFAULT_IMAGE.clone()),
                alpha_mode: AlphaMode::Blend,
                emissive: Srgba::WHITE.with_alpha(0.3).into(),
                ..Default::default()
            })),
            InheritedVisibility::VISIBLE,
        ))
        .id();

    commands.entity(root).add_child(e);
}

fn point_text_2d_towards_camera(
    text: Query<&mut Transform, With<TextBubble>>,
    camera: Single<&Transform, (With<Camera3d>, Without<TextBubble>)>,
) {
    for mut tf in text {
        let dir = tf.translation - camera.translation;
        tf.look_to(dir.with_y(0.0), Vec3::Y);
    }
}

fn sync_transforms_to_parent(
    text: Query<(&mut Transform, &TextBubble)>,
    ducks: Query<&Transform, Without<TextBubble>>,
) -> Result {
    for (mut tf, bubble) in text {
        let parent = ducks.get(bubble.parent)?;
        tf.translation = parent.translation.with_y(0.0) + bubble.offset;
    }
    Ok(())
}
