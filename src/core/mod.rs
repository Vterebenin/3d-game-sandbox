use avian3d::{math::Scalar, prelude::*};
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use blenvy::*;
use crate::character_controller::{CharacterControllerBundle, CharacterControllerPlugin};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CharacterControllerPlugin)
            .register_type::<Player>()
            .register_type::<PlayerCameraFix>()
            .register_type::<Respawnable>()
            .add_systems(Startup, setup)
            .add_systems(Startup, cursor_grab)
            .add_systems(Update, respawn_player)
            .add_systems(Update, rotate_camera);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Respawnable;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct PlayerCameraFix {
    x: f32,
    y: f32,
    z: f32,
    distance: f32,
    yaw: f32,
    pitch: f32,
}

const TEST_WORLD_PATH: &str = "levels/World.glb";
const PLAYER_BLUEPRINT_PATH: &str = "blueprints/Player.glb";

fn setup(mut commands: Commands) {
    commands.spawn((
        BlueprintInfo::from_path(TEST_WORLD_PATH),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
    commands.spawn((
        Player,
        Respawnable,
        BlueprintInfo::from_path(PLAYER_BLUEPRINT_PATH), // mandatory !!
        SpawnBlueprint,                                    // mandatory !!
        Transform::from_xyz(4., 8., 0.),                   // VERY important !!
        CharacterControllerBundle::new(Collider::capsule(0.4, 1.2)).with_movement(
            40.0,
            0.93,
            10.0,
            (30.0 as Scalar).to_radians(),
        ),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        GravityScale(2.0),
    ));
}

// TODO: this is primary for debugging purposes
fn respawn_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut LinearVelocity), (With<Respawnable>, With<Player>)>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut camera_query: Query<
        (&mut Transform, &mut PlayerCameraFix),
        (With<Camera>, Without<Player>),
    >,
) {
    if query.is_empty() || camera_query.is_empty() {
        return;
    }
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
        let (mut camera_transform, mut player_camera) = camera_query.get_single_mut().unwrap();
        camera_transform.translation = Vec3::new(0., 5., 10.);
        player_camera.yaw = 1.53;
        player_camera.pitch = 0.22;
    }
}

fn cursor_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;

    primary_window.cursor_options.visible = false;
}

type WithPlayer = (With<Player>, Without<Camera>);
type WithCamera = (With<Camera>, Without<Player>);

// System to rotate the camera around the player using mouse input
fn rotate_camera(
    mut camera_query: Query<(&mut Transform, &mut PlayerCameraFix), WithCamera>,
    mut player_query: Query<(&mut Transform, Entity), WithPlayer>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    time: Res<Time>,
    physics: SpatialQuery, // Assuming you have a physics engine resource set up.
) {
    if player_query.is_empty() || camera_query.is_empty() {
        return;
    }

    const MOUSE_SENSITIVITY_X: f32 = 0.002;
    const MOUSE_SENSITIVITY_Y: f32 = 0.002;
    const ZOOM_SPEED: f32 = 2.0;

    let (mut camera_transform, mut camera) = camera_query.get_single_mut().unwrap();
    let (mut player_transform, player_id) = player_query.get_single_mut().unwrap();
    // Rotate the camera with mouse drag
    for event in mouse_motion_events.read() {
        let x_delta = event.delta.x;
        let y_delta = event.delta.y;
        camera.yaw += x_delta * MOUSE_SENSITIVITY_X;
        camera.pitch += y_delta * MOUSE_SENSITIVITY_Y;
    }

    // Adjust the camera's zoom
    for event in mouse_wheel.read() {
        camera.distance -= event.y * ZOOM_SPEED * time.delta_secs();
        camera.distance = camera.distance.clamp(2.0, 15.0); // Clamp zoom levels
    }

    // Compute the new camera position based on yaw, pitch, and distance
    let offset = Vec3::new(
        camera.distance * camera.yaw.cos() * camera.pitch.cos(),
        camera.distance * camera.pitch.sin(),
        camera.distance * camera.yaw.sin() * camera.pitch.cos(),
    );

    // Position the camera relative to the player and look at the player
    let desired_position = player_transform.translation + offset;

    let direction = desired_position - player_transform.translation;
    let query_filter = SpatialQueryFilter::from_mask(0b1011).with_excluded_entities([player_id]);

    if let Ok(direction) = Dir3::new(direction.normalize()) {
        if let Some(hit) = physics.cast_shape(
            &Collider::sphere(0.5),
            player_transform.translation, // Start point
            Quat::IDENTITY,
            direction,
            &ShapeCastConfig {
                max_distance: 15.,
                target_distance: 0.,
                ignore_origin_penetration: true,
                ..Default::default()
            },
            &query_filter,
        ) {
            camera_transform.translation = hit.point1;
        } else {
            camera_transform.translation = desired_position;
        }
    } else {
        camera_transform.translation = desired_position;
    }
    camera_transform.look_at(player_transform.translation, Vec3::Y);

    let current_rotation = (camera_transform.rotation).to_euler(EulerRot::YXZ).0;

    // Update the player's rotation
    player_transform.rotation = Quat::from_euler(EulerRot::YXZ, current_rotation, 0.0, 0.0);
}
