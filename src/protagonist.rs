use bevy::prelude::*;
use crate::components::*;

pub struct ProtagonistPlugin;

impl Plugin for ProtagonistPlugin {
    fn build(&self, app:&mut App) {
        app
            .add_systems(Startup, spawn_protagonist);
    }
}

fn spawn_protagonist(mut commands: Commands, 
    sprite_sheets: Res<SpriteSheetHandles>
) {
    let animation_indices = AnimationIndices { first: 24, last: 26 };


    commands.spawn((SpriteSheetBundle {
        texture_atlas: sprite_sheets.characters.clone(),
        sprite: TextureAtlasSprite::new(animation_indices.first),
        ..default()
        },
       animation_indices,
       AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
    );
}