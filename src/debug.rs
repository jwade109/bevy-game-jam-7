#![allow(unused)]

use bevy::color::palettes::css::*;
use bevy::prelude::*;

use crate::ducks::*;

pub fn debug_plugin(app: &mut App) {
    app.add_systems(Update, draw_origin);
    app.add_systems(Update, draw_all_target_positions);
    // app.add_systems(Update, draw_all_transforms);
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

fn draw_all_target_positions(mut gizmos: Gizmos, ducks: Query<(&Transform, &TargetPosition)>) {
    for (tf, tp) in ducks {
        gizmos.line(tf.translation, tp.pos, RED);
    }
}
