use bevy::prelude::*;

pub fn despawn_after_plugin(app: &mut App) {
    app.add_systems(FixedUpdate, update_despawn_after);
}

#[derive(Component, Clone, Copy, Debug)]
pub struct DespawnAfter {
    age: std::time::Duration,
    expiration_time: std::time::Duration,
}

impl DespawnAfter {
    pub fn new(dur: std::time::Duration) -> Self {
        Self {
            age: std::time::Duration::ZERO,
            expiration_time: dur,
        }
    }
}

fn update_despawn_after(
    mut commands: Commands,
    des: Query<(Entity, &mut DespawnAfter)>,
    time: Res<Time<Fixed>>,
) {
    let dt = time.delta();
    for (e, mut des) in des {
        des.age += dt;
        if des.age >= des.expiration_time {
            commands.entity(e).despawn();
        }
    }
}
