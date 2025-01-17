use bevy::prelude::*;
use blenvy::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BlenvyPlugin::default()))
        // We need to register components to make them visible to Blenvy
        .register_type::<Player>()
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Player {
    strength: f32,
    perception: f32,
    endurance: f32,
    charisma: f32,
    intelligence: f32,
    agility: f32,
    luck: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn((
        BlueprintInfo::from_path("levels/World.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
}

