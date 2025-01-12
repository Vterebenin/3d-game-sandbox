use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, print_ball_altitude)
        .add_systems(Update, respawn_cube_on_space)
        .add_systems(Update, camera_handle)
        .add_systems(Update, update_controls)
        .run();
}

#[derive(Component)]
struct Respawnable;

#[derive(Component)]
struct MyCamera;

#[derive(Component)]
struct Player;

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    // commands.spawn(Camera3dBundle {
    //     ..Default::default()
    // });
    commands.spawn((
        MyCamera,
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            ..PerspectiveProjection::default()
        }),
        Transform::from_xyz(-10.0, 10.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /* Create the ground. */
    commands
        .spawn((
            Collider::cuboid(100.0, 0.1, 100.0),
            Mesh3d(meshes.add(Cuboid::new(200.0, 0.2, 200.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
        ))
        .insert(Transform::from_xyz(0.0, 0.0, 0.0));

    /* Create the bouncing ball. */
    commands
        .spawn((
            Player,
            Respawnable,
            RigidBody::Dynamic,
            MeshMaterial3d(materials.add(Color::WHITE)),
            Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
            Transform::from_xyz(0.0, 4.0, 0.0),
            Velocity {
                linvel: Vec3::new(0.0, 0.0, 0.0),
                angvel: Vec3::new(0.0, 0.0, 0.0),
            },
        ))
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Restitution::coefficient(0.7));
}

fn print_ball_altitude(mut positions: Query<&mut Transform, With<RigidBody>>) {
    for mut transform in positions.iter_mut() {
        // dbg!(transform.rotation.to_axis_angle());
        transform.rotation = Quat::from_rotation_z(270_f32.to_radians());
        //println!("Ball altitude: {}", transform.translation.y);
    }
}

fn respawn_cube_on_space(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Respawnable>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (mut transform, mut velocity) in query.iter_mut() {
            // Reset the position and velocity of the cube
            transform.translation = Vec3::new(0.0, 8.0, 0.0);
            velocity.linvel = Vec3::ZERO;
            velocity.angvel = Vec3::ZERO;
        }
    }
}

fn update_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let dirs: HashMap<KeyCode, Vec3> = HashMap::from([
        (KeyCode::KeyW, Vec3::new(1., 0., 0.)),
        (KeyCode::KeyS, Vec3::new(-1., 0., 0.)),
        (KeyCode::KeyA, Vec3::new(0., 0., -1.)),
        (KeyCode::KeyD, Vec3::new(0., 0., 1.)),
    ]);
    for (key, value) in dirs.into_iter() {
        if keyboard_input.pressed(key) || keyboard_input.just_pressed(key) {
            for mut velocity in query.iter_mut() {
                // Reset the position and velocity of the cube
                velocity.linvel += value;
                velocity.angvel = Vec3::ZERO;
            }
        }
    }
}

fn camera_handle(
    player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MyCamera>, Without<Player>)>,
) {
    let player_transform = player_query.get_single().expect("player should exist");  
    let mut camera_transform = camera_query.get_single_mut().expect("camera should exist");  
    // Reset the position and velocity of the cube
    camera_transform.translation = Vec3::new(
        player_transform.translation.x - 10.0,
        player_transform.translation.y + 10.0,
        player_transform.translation.z + 40.0,
    );
}
