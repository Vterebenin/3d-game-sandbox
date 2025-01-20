use avian3d::prelude::*;
use bevy::{prelude::*, utils::hashbrown::HashMap};
use blenvy::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BlenvyPlugin::default(),
            PhysicsPlugins::default(),
        ))
        .register_type::<Player>()
        .register_type::<PlayerCamera>()
        .register_type::<Respawnable>()
        .add_systems(Startup, setup)
        .add_systems(PostStartup, add_physics)
        .add_systems(Update, respawn_player)
        .add_systems(Update, update_controls)
        .add_systems(Update, update_camera)
        .add_systems(Update, camera_handle)
        // .add_systems(Update, camera_collision_prevention_system)
        .run()
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Player;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Respawnable;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct PlayerCamera {
    x: f32,
    y: f32,
    z: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn((
        BlueprintInfo::from_path("levels/World.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
}

fn respawn_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut LinearVelocity), (With<Respawnable>, With<Player>)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (mut transform, mut velocity) in query.iter_mut() {
            transform.translation = Vec3::new(0.0, 8.0, 0.0);
            velocity.x = 0.;
            velocity.y = 0.;
            velocity.z = 0.;
        }
    }
}
fn update_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut LinearVelocity, With<Player>>,
) {
    let dirs: HashMap<KeyCode, Vec3> = HashMap::from([
        (KeyCode::KeyW, Vec3::new(0.5, 0., 0.)),
        (KeyCode::KeyS, Vec3::new(-0.5, 0., 0.)),
        (KeyCode::KeyA, Vec3::new(0., 0., -0.5)),
        (KeyCode::KeyD, Vec3::new(0., 0., 0.5)),
    ]);
    for (key, value) in dirs.into_iter() {
        if keyboard_input.pressed(key) || keyboard_input.just_pressed(key) {
            for mut velocity in query.iter_mut() {
                // Reset the position and velocity of the cube
                velocity.x += value.x;
                velocity.y += value.y;
                velocity.z += value.z;
            }
        }
    }
}
fn camera_collision_prevention_system(
    mut query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    physics: SpatialQuery, // Assuming you have a physics engine resource set up.
) {
    if player_query.is_empty() {
        return;
    }
    let player_transform = player_query.single().translation;

    for mut camera_transform_mut in query.iter_mut() {
        let camera_position = camera_transform_mut.translation;
        let direction = (camera_position - player_transform).normalize();
        if let Ok(direction) = Dir3::new(direction) {
            let max_distance = (camera_position - player_transform).length();
            if let Some(collision) = physics.cast_ray(
                player_transform, // Start point
                direction,
                max_distance, // Max distance of the ray
                true,
                &SpatialQueryFilter::default(),
            ) {
                dbg!("we are in");
                // Adjust the camera position based on the collision
                camera_transform_mut.translation = collision.distance + collision.normal * 0.2; // Add a small buffer distance.
            }
        }
    }
}

fn update_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut PlayerCamera, With<Camera>>,
) {
    let dirs: HashMap<KeyCode, Vec3> = HashMap::from([
        (KeyCode::ArrowUp, Vec3::new(0.15, 0., 0.)),
        (KeyCode::ArrowDown, Vec3::new(-0.15, 0., 0.)),
        (KeyCode::ArrowLeft, Vec3::new(0., 0., -0.15)),
        (KeyCode::ArrowRight, Vec3::new(0., 0., 0.15)),
        (KeyCode::PageUp, Vec3::new(0., 0.15, 0.)),
        (KeyCode::PageDown, Vec3::new(0., -0.15, 0.)),
    ]);
    for (key, value) in dirs.into_iter() {
        if keyboard_input.pressed(key) || keyboard_input.just_pressed(key) {
            for mut camera in query.iter_mut() {
                // Reset the position and velocity of the cube
                camera.x += value.x;
                camera.y += value.y;
                camera.z += value.z;
            }
        }
    }
}

fn camera_handle(
    player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<
        (&mut Transform, &PlayerCamera),
        (With<Camera>, Without<Player>),
    >,
) {
    dbg!("{} | {}", player_query.is_empty(), camera_query.is_empty());
    if player_query.is_empty() || camera_query.is_empty() {
        return;
    }
    let player_transform = player_query.get_single().expect("player should exist");
    let (mut camera_transform, camera) =
        camera_query.get_single_mut().expect("camera should exist");
    // Reset the position and velocity of the cube
    camera_transform.look_at(
        Vec3::new(
            player_transform.translation.x,
            player_transform.translation.y,
            player_transform.translation.z,
        ),
        Vec3::Y,
    );
    camera_transform.translation = Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y,
        player_transform.translation.z,
    ) + Vec3::new(camera.x, camera.y, camera.z);
}

fn add_physics(
    mut commands: Commands,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<Entity, With<Player>>,
) {
    for mut transform in camera_query.iter_mut() {
        transform.translation = Vec3::new(-28., 8.1, 0.);
    }
    for entity_id in player_query.iter() {
        commands
            .entity(entity_id)
            .insert(LinearVelocity::default());
    }
}
