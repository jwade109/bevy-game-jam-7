use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;

pub fn player_plugin(app: &mut App) {
    app.add_systems(Startup, add_player_duck);
    app.add_systems(Update, handle_player_keys);
    app.add_systems(FixedUpdate, propagate_player_physics);
}

#[derive(Component, Default, Debug)]
pub struct PlayerDuck {
    pub velocity: Vec3,
}

fn add_player_duck(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let body = meshes.add(Capsule3d::new(0.5, 2.0));
    let material = materials.add(StandardMaterial::from_color(YELLOW_400));

    commands.spawn((
        PlayerDuck::default(),
        Mesh3d(body),
        MeshMaterial3d(material),
        Transform::default(),
    ));
}

fn handle_player_keys(
    keys: Res<ButtonInput<KeyCode>>,
    mut duck: Query<(&mut PlayerDuck, &Transform)>,
) -> Result {
    let mut velocity = Vec3::ZERO;

    let (mut duck, transform) = duck.single_mut()?;

    if keys.pressed(KeyCode::KeyW) {
        velocity += transform.forward().as_vec3()
    }
    if keys.pressed(KeyCode::KeyA) {
        velocity += transform.left().as_vec3()
    }
    if keys.pressed(KeyCode::KeyS) {
        velocity += transform.back().as_vec3()
    }
    if keys.pressed(KeyCode::KeyD) {
        velocity += transform.right().as_vec3()
    }

    duck.velocity = velocity;

    Ok(())
}

fn propagate_player_physics(duck: Query<(&PlayerDuck, &mut Transform)>, time: Res<Time<Fixed>>) {
    let dt = time.delta_secs();
    for (duck, mut transform) in duck {
        transform.translation += duck.velocity * dt;
    }
}
