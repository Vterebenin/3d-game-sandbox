use avian3d::{math::Scalar, prelude::*};
use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    utils::hashbrown::HashMap,
    window::{CursorGrabMode, PrimaryWindow},
};
use blenvy::*;
use character_controller::{CharacterControllerBundle, CharacterControllerPlugin};
mod character_controller;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BlenvyPlugin::default(),
            PhysicsPlugins::default(),
            CharacterControllerPlugin,
            PhysicsDebugPlugin::default(),
        ))
        .register_type::<Player>()
        .register_type::<PlayerCameraFix>()
        .register_type::<Respawnable>()
        .add_systems(Startup, setup)
        // .add_systems(Startup, hide_cursor)
        .add_systems(Startup, cursor_grab)
        .add_systems(Update, respawn_player)
        // .add_systems(Update, update_camera)
        // .add_systems(Update, camera_handle)
        // .add_systems(Update, camera_follow)
        .add_systems(Update, rotate_camera)
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
    distance: f32,
    yaw: f32,
    pitch: f32,
}

impl PlayerCameraFix {
    fn get_offset(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
    fn set_offset(&mut self, vec: Vec3) {
        self.x = vec.x;
        self.y = vec.y;
        self.z = vec.z;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        BlueprintInfo::from_path("levels/World.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
    commands.spawn((
        Player,
        Respawnable,
        BlueprintInfo::from_path("blueprints/Player.glb"), // mandatory !!
        SpawnBlueprint,                                    // mandatory !!
        Transform::from_xyz(0., 8., 0.),                   // VERY important !!
        CharacterControllerBundle::new(Collider::capsule(0.4, 1.2)).with_movement(
            30.0,
            0.92,
            7.0,
            (30.0 as Scalar).to_radians(),
        ),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        GravityScale(2.0),
    ));
}

fn respawn_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut LinearVelocity), (With<Respawnable>, With<Player>)>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let mut primary_window = q_windows.single_mut();
        if primary_window.cursor_options.grab_mode == CursorGrabMode::Locked {
            primary_window.cursor_options.grab_mode = CursorGrabMode::None;
            primary_window.cursor_options.visible = true;
        } else {
            cursor_grab(q_windows);
        }
    }
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        for (mut transform, mut velocity) in query.iter_mut() {
            transform.translation = Vec3::new(0.0, 8.0, 0.0);
            velocity.x = 0.;
            velocity.y = 0.;
            velocity.z = 0.;
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

// fn update_camera(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut query: Query<&mut PlayerCameraFix, With<Camera>>,
// ) {
//     let dirs: HashMap<KeyCode, Vec3> = HashMap::from([
//         (KeyCode::ArrowUp, Vec3::new(0.15, 0., 0.)),
//         (KeyCode::ArrowDown, Vec3::new(-0.15, 0., 0.)),
//         (KeyCode::ArrowLeft, Vec3::new(0., 0., -0.15)),
//         (KeyCode::ArrowRight, Vec3::new(0., 0., 0.15)),
//         (KeyCode::PageUp, Vec3::new(0., 0.15, 0.)),
//         (KeyCode::PageDown, Vec3::new(0., -0.15, 0.)),
//     ]);
//     for (key, value) in dirs.into_iter() {
//         if keyboard_input.pressed(key) || keyboard_input.just_pressed(key) {
//             for mut camera in query.iter_mut() {
//                 // Reset the position and velocity of the cube
//                 camera.x += value.x;
//                 camera.y += value.y;
//                 camera.z += value.z;
//             }
//         }
//     }
// }

// fn camera_handle(
//     player_query: Query<&mut Transform, With<Player>>,
//     mut camera_query: Query<(&mut Transform, &PlayerCameraFix), (With<Camera>, Without<Player>)>,
// ) {
//     if player_query.is_empty() || camera_query.is_empty() {
//         return;
//     }
//     let player_transform = player_query.get_single().expect("player should exist");
//     let (mut camera_transform, camera) =
//         camera_query.get_single_mut().expect("camera should exist");
//     // Reset the position and velocity of the cube
//     camera_transform.look_at(
//         Vec3::new(
//             player_transform.translation.x,
//             player_transform.translation.y,
//             player_transform.translation.z,
//         ),
//         Vec3::Y,
//     );
//     camera_transform.translation = Vec3::new(
//         player_transform.translation.x,
//         player_transform.translation.y,
//         player_transform.translation.z,
//     ) + Vec3::new(camera.x, camera.y, camera.z);
// }

fn cursor_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    // let mut primary_window = q_windows.single_mut();

    // // primary_window.cursor_options.grab_mode = CursorGrabMode::Confined;

    // primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;

    // primary_window.cursor_options.visible = false;
}

// // System to make the camera follow the player
// fn camera_follow(
//     player_query: Query<&Transform, With<Player>>,
//     mut camera_query: Query<(&mut Transform, &PlayerCameraFix), (With<Camera>, Without<Player>)>,
// ) {
//     if player_query.is_empty() || camera_query.is_empty() {
//         return;
//     }
//     let player_transform = player_query.single();
//     let (mut camera_transform, camera) = camera_query.single_mut();
// 
//     // Update the camera's position based on the player's position and the fixed offset
//     camera_transform.translation = player_transform.translation + camera.get_offset();
// }

// System to rotate the camera around the player using mouse input
fn rotate_camera(
    mut camera_query: Query<(&mut Transform, &mut PlayerCameraFix), With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    time: Res<Time>,
) {
    if player_query.is_empty() || camera_query.is_empty() {
        return;
    }
    const MOUSE_SENSITIVITY_X: f32 = 0.0000001;
    const MOUSE_SENSITIVITY_Y: f32 = 0.000001;
    const ZOOM_SPEED: f32 = 2.0;

    if let Ok((mut camera_transform, mut camera)) = camera_query.get_single_mut() {
        if let Ok(player_transform) = player_query.get_single() {
            // Rotate the camera with mouse drag
            for event in mouse_motion_events.read() {
                println!("{} {}", event.delta.x, event.delta.y);
                camera.yaw -= event.delta.x * MOUSE_SENSITIVITY_X;
                camera.pitch += event.delta.y * MOUSE_SENSITIVITY_Y;
            }

            // Adjust the camera's zoom
            for event in mouse_wheel.read() {
                camera.distance -= event.y * ZOOM_SPEED * time.delta_secs();
                camera.distance = camera.distance.clamp(2.0, 15.0); // Clamp zoom levels
            }

            // Compute the new camera position based on yaw, pitch, and distance
            let offset = Vec3::new(
                camera.distance
                    * camera.yaw.cos()
                    * camera.pitch.cos(),
                camera.distance * camera.pitch.sin(),
                camera.distance
                    * camera.yaw.sin()
                    * camera.pitch.cos(),
            );


            // Position the camera relative to the player and look at the player
            camera_transform.translation = player_transform.translation + offset;
            camera_transform.look_at(player_transform.translation, Vec3::Y);
        }
    }
}
