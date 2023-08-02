use bevy::prelude::*;
use bevy::input::gamepad::*;

use crate::components::*;

pub struct ControllerInputPlugin;

impl Plugin for ControllerInputPlugin {
    fn build(&self, app:&mut App) {
        app
            .add_systems(Update, gamepad_connections)
            .add_systems(Update, gamepad_input);
    }
}

#[derive(Resource)]
struct MyGamepad(Gamepad);

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut events: EventReader<GamepadConnectionEvent>
) {
    for ev in events.iter() {
        let id = ev.gamepad;
        if ev.connected() && my_gamepad.is_none() {
            commands.insert_resource(MyGamepad(id));
        }
        if ev.disconnected(){
            if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                if *old_id == id {
                    commands.remove_resource::<MyGamepad>();
                }
            }
        }
    }
}

fn gamepad_input(
    mut player_query: Query<&mut Player>,
    axes: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        gp.0
    } else {
        return;
    };

    let axis_lx = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::LeftStickX
    };
    let axis_ly = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::LeftStickY
    };

    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        let mut player = player_query.single_mut();
        player.stick_pos = Vec2::new(x, y)
    }
}