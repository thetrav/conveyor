
use bevy::window::PrimaryWindow;
use bevy::input::mouse::MouseWheel;

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
    mut windows_q: Query<&mut Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut cursor_q: Query<(
        &CursorTool,
        &mut Transform,
    )>,
) {
    let (camera, camera_transform) = camera_q.single();
    for mut window in &mut windows_q {
        if let Some(position) = window.cursor_position()
            .and_then(|cp| camera.viewport_to_world(camera_transform, cp)
                .map(|ray| ray.origin.truncate())) {
            window.cursor.visible = false;
            for (_, mut xform) in &mut cursor_q {
                xform.translation.x = position.x - (position.x % TILE_WIDTH);
                xform.translation.y = position.y - (position.y % TILE_HEIGHT);
            }
        } else {
            window.cursor.visible = true;
        }
    }
}

pub struct BuilderPlugin;

impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, follow_mouse)
            .add_systems(Update, tool_selection);
    }
}
