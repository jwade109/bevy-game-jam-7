#![allow(unused)]

use bevy::color::palettes::css::*;
use bevy::prelude::*;

use crate::ducks::*;

pub fn debug_plugin(app: &mut App) {
    app.add_systems(Update, draw_origin);
    app.add_systems(
        Update,
        (draw_all_target_positions, draw_all_separation_forces),
    );
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
        let p = tf.translation.with_y(2.0);
        let q = tp.pos.with_y(2.0);
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

fn draw_all_separation_forces(mut gizmos: Gizmos, sep: Query<(&Transform, &Separation)>) {
    for (tf, sep) in sep {
        let p = tf.translation.with_y(1.5);
        let q = p - sep.force;
        gizmos.arrow(p, q, PURPLE);
    }
}
