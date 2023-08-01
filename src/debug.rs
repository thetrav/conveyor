use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::input::common_conditions::input_toggle_active;


use crate::components::*;

pub struct DebugPlugin;


impl Plugin for DebugPlugin {
    fn build(&self, app:&mut App) {
        app.add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape))
        )
        .register_type::<Player>();
    }
}