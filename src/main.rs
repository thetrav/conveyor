use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::input::mouse::MouseWheel;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

const FACTORY_TILEMAP: &str = "kenney/pixel_platformer/factory_expansion/tilemap.png";
const CHARACTER_TILEMAP: &str = "kenney/pixel_platformer/characters_packed.png";

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component)]
struct CursorTool {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct MainCamera;



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

    let animation_indices = AnimationIndices { first: 24, last: 26 };

    commands.spawn((Camera2dBundle::default(), MainCamera));

    commands.spawn((SpriteSheetBundle {
        texture_atlas: factory_atlas_handle,
        sprite: TextureAtlasSprite::new(85),
        transform: Transform::from_xyz(50.0, 5.0, 0.0),
        ..default()
    }, CursorTool {first: 84, last: 86}));

    commands.spawn((SpriteSheetBundle {
        texture_atlas: character_atlas_handle,
        sprite: TextureAtlasSprite::new(animation_indices.first),
        ..default()
        },
       animation_indices,
       AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
    );
}


fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}


fn tool_selection(
    mut scroll_evr: EventReader<MouseWheel>,
    mut cursor_q: Query<(
        &CursorTool,
        &mut TextureAtlasSprite,
    )>,
) {
    for (cursor_tool, mut sprite) in &mut cursor_q {
        use bevy::input::mouse::MouseScrollUnit;
        for ev in scroll_evr.iter() {
            match ev.unit {
                MouseScrollUnit::Line => {
                    println!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
                    let size = (cursor_tool.last - cursor_tool.first + 1) as f32;
                    let remainder = ev.y % size;
                    let mut ind = sprite.index as f32 + remainder;
                    println!("remainder: {}, ind: {}", remainder, ind);
                    if ind > cursor_tool.last as f32 {
                        ind -= size as f32;
                    }
                    if ind < cursor_tool.first as f32 {
                        ind += size;
                    }
                    println!("ind: {}",ind);
                    sprite.index = ind as usize;
                }
                MouseScrollUnit::Pixel => {
                    println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
                }
            }
        }
    }
}

fn follow_mouse(
    windows_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut cursor_q: Query<(
        &CursorTool,
        &mut Transform,
    )>,
) {
    let (camera, camera_transform) = camera_q.single();
    if let Some(position) = windows_q.single().cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        for (_, mut xform) in &mut cursor_q {
            xform.translation.x = position.x;
            xform.translation.y = position.y;
        }
    }
}

pub struct ConveyorPlugin;

impl Plugin for ConveyorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, animate_sprite)
            .add_systems(Update, follow_mouse)
            .add_systems(Update, tool_selection);
    }
}
