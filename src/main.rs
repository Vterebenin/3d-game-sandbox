use std::any::Any;

use avian3d::{math::Vector, prelude::*};
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
        .register_type::<PlayerCameraFix>()
        .register_type::<Respawnable>()
        .add_systems(Startup, setup)
        .add_systems(PostStartup, add_physics)
        .add_systems(Update, respawn_player)
        .add_systems(Update, update_controls)
        .add_systems(Update, update_camera)
        .add_systems(Update, camera_handle)
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
struct PlayerCameraFix {
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
            dbg!("found the player!!");
            println!("found the player!!");
            for mut velocity in query.iter_mut() {
                // Reset the position and velocity of the cube
                velocity.x += value.x;
                velocity.y += value.y;
                velocity.z += value.z;
            }
        }
    }
}

fn update_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut PlayerCameraFix, With<PlayerCameraFix>>,
) {
    let dirs: HashMap<KeyCode, Vec3> = HashMap::from([
        (KeyCode::ArrowUp, Vec3::new(0.05, 0., 0.)),
        (KeyCode::ArrowDown, Vec3::new(-0.05, 0., 0.)),
        (KeyCode::ArrowLeft, Vec3::new(0., 0., -0.05)),
        (KeyCode::ArrowRight, Vec3::new(0., 0., 0.05)),
        (KeyCode::PageUp, Vec3::new(0., 0.05, 0.)),
        (KeyCode::PageDown, Vec3::new(0., -0.05, 0.)),
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
    mut camera_query: Query<(&mut Transform, &PlayerCameraFix), (With<PlayerCameraFix>, Without<Player>)>,
) {
    if player_query.is_empty() || camera_query.is_empty() {
        return;
    }
    let player_transform = player_query.get_single().expect("player should exist");
    let (mut camera_transform, camera) =
        camera_query.get_single_mut().expect("camera should exist");
    // Reset the position and velocity of the cube
    camera_transform.look_at(
        Vec3::new(
            player_transform.translation.x + 10.,
            player_transform.translation.y + 1.5,
            player_transform.translation.z + 0.0,
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
    player_query: Query<Entity, With<Player>>,
) {
    for entity_id in player_query.iter() {
        commands.entity(entity_id)
            .insert(LinearVelocity::default());
    }
}
