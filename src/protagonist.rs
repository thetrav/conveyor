use bevy::prelude::*;
use crate::components::*;

pub struct ProtagonistPlugin;

impl Plugin for ProtagonistPlugin {
    fn build(&self, app:&mut App) {
        app
            .add_systems(Startup, spawn_protagonist)
            .add_systems(Update, camera_follow);
    }
}

fn camera_follow(player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>) {
        let player_transform = player_query.single();
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }

fn spawn_protagonist(mut commands: Commands, 
    sprite_sheets: Res<SpriteSheetHandles>
) {
    let animation_indices = AnimationIndices { first: 24, last: 26 };

    let player = Player{
        speed: 1.0,
        up: false,
        down: false,
        left: false,
        right: false,
        stick_pos: Vec2::splat(1.0)
       };

    commands.spawn((SpriteSheetBundle {
        texture_atlas: sprite_sheets.characters.clone(),
        sprite: TextureAtlasSprite::new(animation_indices.first),
        ..default()
       },
       animation_indices,
       AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
       player
    ));
}