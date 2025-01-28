
use avian3d::prelude::*;
use bevy::prelude::*;
use blenvy::*;

mod character_controller;
mod core;

use core::CorePlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin::default())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        position: WindowPosition::Automatic,
                        // mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        visible: true,
                        ..default()
                    }),
                    ..default()
                }),
            BlenvyPlugin::default(),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            CorePlugin,
        ))
        .run()
}
