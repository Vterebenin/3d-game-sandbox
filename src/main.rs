use avian3d::prelude::*;
use bevy::prelude::*;
use blenvy::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BlenvyPlugin::default(),
            PhysicsPlugins::default(),
        ))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn((
        BlueprintInfo::from_path("levels/World.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
}
