pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use crate::systems::*;
use components::*;
use events::*;
use resources::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                //turn off image filtering for pixel style
                .set(ImagePlugin::default_nearest())
                //set window title and size
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "'The Binding of Isaac'".into(),
                        resolution: (1920.0, 1080.0).into(),
                        resizable: true,
                        mode: bevy::window::WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_startup_system(yep_camera)
        .add_startup_system(yep_player)
        .add_startup_system(yep_enemy)
        .add_system(yep_move)
        .add_system(stay_on_screen_challenge)
        .add_system(yep_move_enemy)
        .add_system(dont_stick_to_walls_challenge)
        .add_system(confine_enemy)
        .add_system(collision_l)
        .add_system(yep_tear)
        .add_system(yep_move_tear)
        .add_system(collision_2)
        .run();
}
