use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

const FACTORY_TILEMAP: &str = "kenney/pixel_platformer/factory_expansion/tilemap.png";
const CHARACTER_TILEMAP: &str = "kenney/pixel_platformer/characters_packed.png";

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()),
                      ConveyorPlugin))
        .run();
}

fn setup(mut commands: Commands,
         asset_server: Res<AssetServer>,
         mut texture_atlases: ResMut<Assets<TextureAtlas>>) {

    let factory_atlas_handle = texture_atlases.add(
        TextureAtlas::from_grid(asset_server.load(FACTORY_TILEMAP),
                                       Vec2::new(18.0, 18.0),
                                       16, 7,
                                       Option::from(Vec2::new(1.0, 1.0)),
                                       None));
    let character_atlas_handle = texture_atlases.add(
        TextureAtlas::from_grid(asset_server.load(CHARACTER_TILEMAP),
                                Vec2::new(24.0, 24.0),
                                9, 3,
                                None,
                                None));

    commands.spawn(Camera2dBundle::default());

    // commands.spawn(SpriteSheetBundle {
    //     texture_atlas: factory_atlas_handle,
    //     sprite: TextureAtlasSprite::new(0),
    //     transform: Transform::from_scale(Vec3::splat(6.0)),
    //     ..default()
    // });

    commands.spawn(SpriteSheetBundle {
        texture_atlas: character_atlas_handle,
        sprite: TextureAtlasSprite::new(24),
        transform: Transform::from_scale(Vec3::splat(6.0)),
        ..default()
    });
}

pub struct ConveyorPlugin;

impl Plugin for ConveyorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
