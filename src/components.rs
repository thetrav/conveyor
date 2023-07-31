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
    pub speed: f32,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub stick_pos: Vec2
}


#[derive(Resource)]
pub struct SpriteSheetHandles {
    pub characters: Handle<TextureAtlas>,
    pub tiles: Handle<TextureAtlas>
}