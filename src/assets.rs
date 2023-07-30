use bevy::prelude::*;
use crate::constants::*;
use crate::components::*;
pub struct AssetsPlugin;

const FACTORY_TILEMAP: &str = "kenney/pixel_platformer/factory_expansion/tilemap.png";
const CHARACTER_TILEMAP: &str = "kenney/pixel_platformer/characters_packed.png";

impl Plugin for AssetsPlugin {
    fn build(&self, app:&mut App) {
        app
            .add_systems(PreStartup, load_assets);
    }
}

fn load_assets(mut commands: Commands,
                   asset_server: Res<AssetServer>,
               mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let factory_atlas_handle = texture_atlases.add(
        TextureAtlas::from_grid(asset_server.load(FACTORY_TILEMAP),
                                Vec2::new(TILE_WIDTH, TILE_HEIGHT),
                                16, 7,
                                Option::from(Vec2::new(1.0, 1.0)),
                                None));
    let character_atlas_handle = texture_atlases.add(
        TextureAtlas::from_grid(asset_server.load(CHARACTER_TILEMAP),
                                Vec2::new(24.0, 24.0),
                                9, 3,
                                None,
                                None));
    commands.insert_resource(SpriteSheetHandles{
        characters: character_atlas_handle,
        tiles: factory_atlas_handle
    });
}