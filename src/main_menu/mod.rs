mod components;
mod styles;
mod systems;

use crate::AppState;
use bevy::prelude::*;
use systems::layout::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            //Enter state systems
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            //Exit state systems
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}
