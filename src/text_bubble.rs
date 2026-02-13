use bevy::prelude::*;
use bevy_rich_text3d::{Text3d, Text3dStyling, TextAtlas};

use crate::{
    despawn_after::DespawnAfter,
    math::{random_range, random_vec},
};

pub fn text_bubble_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            handle_quack_messages,
            point_text_2d_towards_camera,
            sync_transforms_to_parent,
        )
            .chain(),
    );

    app.add_message::<Quack>();
}

#[derive(Debug, Clone, Copy)]
enum QuackKind {
    Noise,
    Info,
}

#[derive(Message, Debug, Clone)]
pub struct Quack {
    pub entity: Entity,
    text: String,
    kind: QuackKind,
}

impl Quack {
    pub fn noise(entity: Entity, text: impl Into<String>) -> Self {
        Self {
            entity,
            text: text.into(),
            kind: QuackKind::Noise,
        }
    }

    pub fn info(entity: Entity, text: impl Into<String>) -> Self {
        Self {
            entity,
            text: text.into(),
            kind: QuackKind::Info,
        }
    }
}

#[derive(Component)]
struct TextBubble {
    parent: Entity,
    offset: Vec3,
}

fn handle_quack_messages(
    mut messages: MessageReader<Quack>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for quack in messages.read() {
        let p = random_vec(2.0, 15.0);
        let tf = Transform::from_xyz(p.x, 4.0, p.y);

        let xz = random_vec(0.03, 0.3);
        let y = random_range(1.2..=2.0);

        let root = commands
            .spawn((
                DespawnAfter::new(std::time::Duration::from_secs(2)),
                TextBubble {
                    parent: quack.entity,
                    offset: Vec3::new(xz.x, y, xz.y),
                },
                tf,
                InheritedVisibility::VISIBLE,
            ))
            .id();

        let color = match quack.kind {
            QuackKind::Info => Srgba::new(1., 1., 1., 1.),
            QuackKind::Noise => Srgba::WHITE.with_alpha(0.4),
        };

        let e = commands
            .spawn((
                Text3d::new(quack.text.clone()),
                Text3dStyling {
                    size: 80.,
                    color,
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
