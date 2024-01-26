use bevy::prelude::*;

#[derive(Component, Default)]
pub struct MainMenuState;

#[derive(Component, Default)]
pub struct GameState;

#[derive(Component)]
pub struct Tear {
    pub direction: Vec2,
}

#[derive(Component, Default)]
pub struct Player {
    pub tear_cooldown: f32,
    pub position: Vec2,
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Enemy {
    pub enemy_direction: Vec2,
}
