use bevy::prelude::*;
use bevy_inspector_egui::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::robot_plugin::ID;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app
                .register_type::<ID>()
                .add_plugins(WorldInspectorPlugin::new());
        }
    }
}