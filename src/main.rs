use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::input::common_conditions::input_toggle_active;

mod assets;
mod constants;
mod components;
mod sprite_animation;
mod protagonist;
mod tiled_loader;

use crate::assets::*;
use crate::sprite_animation::*;
use crate::protagonist::*;

use bevy_ecs_tilemap::prelude::*;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
            AssetsPlugin,
            SpriteAnimationPlugin,
            ProtagonistPlugin,
            TilemapPlugin,
            tiled_loader::TiledMapPlugin)
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let map_handle: Handle<tiled_loader::TiledMap> = asset_server.load("factory.tmx");
    commands.spawn(tiled_loader::TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}
