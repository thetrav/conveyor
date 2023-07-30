use bevy::prelude::*;


#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct CursorTool {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct MainCamera;


#[derive(Default, Component)]
pub struct Player {
    speed: f32,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    stick_pos: Vec2
}


#[derive(Resource)]
pub struct SpriteSheetHandles {
    pub characters: Handle<TextureAtlas>,
    pub tiles: Handle<TextureAtlas>
}