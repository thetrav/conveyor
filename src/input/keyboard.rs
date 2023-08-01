use bevy::prelude::*;

use crate::components::*;

pub struct KeyboardInputPlugin;

impl Plugin for KeyboardInputPlugin {
    fn build(&self, app:&mut App) {
        app
            .add_systems(Update, keyboard_input);
    }
}

fn keyboard_input(
    mut player_query: Query<&mut Player>,
    keyboard: Res<Input<KeyCode>>
){
    let mut player = player_query.single_mut();
    player.up = keyboard.pressed(KeyCode::W);
    player.down = keyboard.pressed(KeyCode::S);
    player.left = keyboard.pressed(KeyCode::A);
    player.right = keyboard.pressed(KeyCode::D);
}