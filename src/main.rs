use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::input::common_conditions::input_toggle_active;

mod assets;
mod constants;
mod components;
mod sprite_animation;
mod protagonist;

use crate::assets::*;
use crate::sprite_animation::*;
use crate::protagonist::*;


fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
            AssetsPlugin,
            SpriteAnimationPlugin,
            ProtagonistPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
