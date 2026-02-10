#![allow(unused)]

use bevy::color::palettes::css::*;
use bevy::prelude::*;

use crate::ducks::*;
use crate::player::PlayerDuck;

pub fn debug_plugin(app: &mut App) {
    app.add_systems(Update, toggle_debug_on_key_p);

    app.add_systems(
        Update,
        (draw_origin, draw_all_target_positions, draw_boid_forces).run_if(is_debug_enabled),
    );

    app.insert_state(DebugState::Enabled);
    // app.add_systems(Update, draw_all_transforms);
}

fn is_debug_enabled(state: Res<State<DebugState>>) -> bool {
    *state == DebugState::Enabled
}

#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum DebugState {
    Enabled,
    Disabled,
}

fn toggle_debug_on_key_p(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<DebugState>>,
    mut next: ResMut<NextState<DebugState>>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        match state.get() {
            DebugState::Enabled => next.set(DebugState::Disabled),
            DebugState::Disabled => next.set(DebugState::Enabled),
        }
    }
}

fn draw_origin(mut gizmos: Gizmos) {
    gizmos.axes(Transform::default(), 5.0);
}

fn draw_all_transforms(mut gizmos: Gizmos, transforms: Query<&GlobalTransform>) {
    for tf in transforms {
        let tf = tf.compute_transform();
        gizmos.axes(tf, 0.5);
    }
}

const DUCK_DEBUG_MARKERS_Y: f32 = 2.0;

fn draw_all_target_positions(mut gizmos: Gizmos, ducks: Query<(&Transform, &TargetPosition)>) {
    for (tf, tp) in ducks {
        let p = tf.translation.with_y(DUCK_DEBUG_MARKERS_Y);
        let q = tp.pos.with_y(DUCK_DEBUG_MARKERS_Y);
        gizmos.line(p, q, BLUE.with_alpha(0.5));
        gizmos.primitive_3d(
            &Sphere::new(0.3),
            Isometry3d::from_translation(p),
            RED.with_alpha(0.4),
        );
        gizmos.primitive_3d(
            &Sphere::new(0.3),
            Isometry3d::from_translation(q),
            GREEN.with_alpha(0.4),
        );
    }
}

fn draw_boid_forces(mut gizmos: Gizmos, sep: Query<(&Transform, &Boid), Without<PlayerDuck>>) {
    for (tf, sep) in sep {
        let p = tf.translation.with_y(DUCK_DEBUG_MARKERS_Y);
        let q = p + sep.separation;
        gizmos.line(p, q, PURPLE);

        let p = tf.translation.with_y(DUCK_DEBUG_MARKERS_Y);
        let q = p + sep.seek_target;
        gizmos.line(p, q, GREEN);

        let p = tf.translation.with_y(DUCK_DEBUG_MARKERS_Y);
        let q = p + sep.total_force();
        gizmos.line(p, q, RED);
    }
}
