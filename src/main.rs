use bevy::prelude::*;

mod assets;
mod constants;
mod components;
mod sprite_animation;
mod protagonist;
mod tiled_loader;
mod wasm_window_size;
mod debug;
mod input;

use crate::assets::*;
use crate::sprite_animation::*;
use crate::protagonist::*;
use crate::input::keyboard::*;
use crate::input::controller::*;

use bevy_ecs_tilemap::prelude::*;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            debug::DebugPlugin,
            AssetsPlugin,
            SpriteAnimationPlugin,
            ProtagonistPlugin,
            TilemapPlugin,
            KeyboardInputPlugin,
            ControllerInputPlugin,
            tiled_loader::TiledMapPlugin,
            wasm_window_size::WindowResizePlugin)
        )
        .add_systems(Startup, setup)
        // .insert_resource(Msaa::Off) // see https://github.com/bevyengine/bevy/issues/3593
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
