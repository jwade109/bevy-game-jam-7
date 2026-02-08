use bevy::color::palettes::tailwind::BLUE_400;
use bevy::prelude::*;
use std::collections::BTreeMap;

pub fn lake_plugin(app: &mut App) {
    app.add_systems(Startup, (setup_resources, add_lake_cells).chain());
    app.add_systems(FixedUpdate, update_cell_heights);

    app.add_observer(on_add_lake_cell);
}

#[derive(Component)]
struct LakeCell;

pub struct LakeIndex(pub IVec2);

struct LakeLookup(BTreeMap<IVec2, Entity>);

pub fn transform_to_index(transform: Transform) -> IVec2 {
    transform.translation.xz().round().as_ivec2()
}

#[derive(Event, Debug)]
struct AddLakeCell {
    location: IVec2,
}

fn on_add_lake_cell(
    event: On<AddLakeCell>,
    mut commands: Commands,
    mesh: Res<CellMesh>,
    mat: Res<CellMaterial>,
) {
    info!("Adding lake cell: {:?}", event);
    let tf = Transform::from_xyz(event.location.x as f32, 0.0, event.location.y as f32);

    let rot = Quat::from_rotation_x(std::f32::consts::PI / 4.0)
        * Quat::from_rotation_z(std::f32::consts::PI / 4.0);

    let rotation = Transform::from_rotation(rot);

    commands.spawn((tf, LakeCell)).with_child((
        Mesh3d(mesh.0.clone()),
        rotation,
        MeshMaterial3d(mat.0.clone()),
    ));
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
    let mat = materials.add(StandardMaterial::from_color(BLUE_400));

    commands.insert_resource(CellMesh(mesh));
    commands.insert_resource(CellMaterial(mat));
}

// fn make_mesh(mut commands: Commands) {
//     let builder = MeshMaker::default();
//     let t = 0.0;
//     for x in -10..=10 {
//         for z in -10..=10 {
//             let y = ((x + z) as f32 / 10.0 + t).sin() * 3.0;
//             let p = Vec3::new(x as f32, y, z as f32);
//             points.push(p);
//         }
//     }
// }

fn add_lake_cells(mut commands: Commands) {
    for x in -50..=50 {
        for z in -50..=50 {
            let event = AddLakeCell {
                location: (x, z).into(),
            };
            commands.trigger(event);
        }
    }
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
