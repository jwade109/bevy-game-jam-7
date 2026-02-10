use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use rand::*;
use std::collections::BTreeMap;

pub fn lake_plugin(app: &mut App) {
    app.add_systems(
        Startup,
        (setup_resources, set_sky_color, add_lake_cells).chain(),
    );
    // app.add_systems(FixedUpdate, update_cell_heights);

    app.add_observer(on_add_lake_cell);
}

#[derive(Component)]
struct LakeCell;

#[allow(unused)]
pub struct LakeIndex(pub IVec2);

#[allow(unused)]
struct LakeLookup(BTreeMap<IVec2, Entity>);

#[allow(unused)]
pub fn transform_to_index(transform: Transform) -> IVec2 {
    transform.translation.xz().round().as_ivec2()
}

#[derive(Event, Debug)]
struct AddLakeCell {
    location: IVec2,
}

#[derive(Event, Debug)]
struct AddLillyPad {
    location: IVec2,
}

fn on_add_lake_cell(
    event: On<AddLakeCell>,
    mut commands: Commands,
    mesh: Res<CellMesh>,
    mat: Res<CellMaterial>,
) {
    info!("Adding lake cell: {:?}", event);
    let tf = Transform::from_xyz(event.location.x as f32, 0.0, event.location.y as f32)
        .with_scale(Vec3::new(1000.0, 0.01, 1000.0));

    let rot = Quat::from_rotation_x(std::f32::consts::PI / 4.0)
        * Quat::from_rotation_z(std::f32::consts::PI / 4.0);

    let rotation = Transform::from_rotation(rot);

    commands.spawn((tf, LakeCell)).with_child((
        Mesh3d(mesh.0.clone()),
        rotation,
        MeshMaterial3d(mat.0.clone()),
    ));
}

fn set_sky_color(mut color: ResMut<ClearColor>)
{
    color.0 = BLUE_300.into();
}

#[derive(Resource)]
struct CellMesh(pub Handle<Mesh>);

#[derive(Resource)]
struct CellMaterial(pub Handle<StandardMaterial>);

fn setup_resources(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::from_length(1.0));
    let mut mat = StandardMaterial::from_color(BLUE_400);
    mat.reflectance = 0.99;
    mat.metallic = 0.99;
    let mat = materials.add(mat);

    commands.insert_resource(CellMesh(mesh));
    commands.insert_resource(CellMaterial(mat));

    let lillypad = meshes.add(Cylinder::new(1.0, 0.05));
    let lillypad_material = materials.add(StandardMaterial::from_color(GREEN_500));

    for _ in 0..200 {
        let x = rand::rng().random_range(-100.0..100.0);
        let z = rand::rng().random_range(-100.0..100.0);
        let tf = Transform::from_xyz(x, 0.0, z);
        commands.spawn((
            tf,
            Mesh3d(lillypad.clone()),
            MeshMaterial3d(lillypad_material.clone()),
        ));
    }
}

fn add_lake_cells(mut commands: Commands) {
    commands.trigger(AddLakeCell {
        location: (0, 0).into(),
    });
}

fn update_cell_heights(cells: Query<&mut Transform, With<LakeCell>>, time: Res<Time<Fixed>>) {
    let t = time.elapsed_secs();
    for mut tf in cells {
        let x = tf.translation.x;
        let z = tf.translation.z;
        let y = ((x + z) as f32 / 10.0 + t).sin() * 3.0;
        tf.translation.y = y;
    }
}
