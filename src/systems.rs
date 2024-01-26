use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use bevy::transform::components::Transform;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 900000001.0;
pub const ENEMY_NUMBER: i32 = 4;
pub const ENEMY_SPEED: f32 = 250.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const TEAR_SPEED: f32 = 200.0;
pub const TEAR_SIZE: f32 = 64.0;
pub const COOLDOWN_CONST: f32 = 1.0;

pub fn yep_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn yep_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player = Player {
        tear_cooldown: 0.0,
        position: Vec2::new(0.0, 0.0),
        direction: Vec2::new(0.0, 0.0),
    };
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            texture: asset_server.load("1image.png"),
            transform: Transform {
                translation: Vec3::new(player.position.x, player.position.y, 0.0),
                ..default()
            },
            ..default()
        },
        player,
    ));
}

pub fn yep_move(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in player_query.iter_mut() {
        let dt = time.delta_seconds();

        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 100.0, 0.0) * PLAYER_SPEED * dt;
        }
        if keyboard.pressed(KeyCode::A) {
            direction += Vec3::new(-100.0, 0.0, 0.0) * PLAYER_SPEED * dt;
        }
        if keyboard.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -100.0, 0.0) * PLAYER_SPEED * dt;
        }
        if keyboard.pressed(KeyCode::D) {
            direction += Vec3::new(100.0, 0.0, 0.0) * PLAYER_SPEED * dt;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction;
        player.position += Vec2::new(direction.x, direction.y);
    }
}

pub fn yep_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..ENEMY_NUMBER {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0)
                    .with_scale(Vec3::new(0.15, 0.15, 1.0)),
                texture: asset_server.load("2image.png"),
                ..default()
            },
            Enemy {
                enemy_direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn stay_on_screen_challenge(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (mut transform, mut player) in player_query.iter_mut() {
        let window = window_query.get_single().unwrap();

        let player_size_by2 = PLAYER_SIZE / 2.0; //32.0
        let x_min = 0.0 + player_size_by2;
        let x_max = window.width() - player_size_by2;
        let y_min = 0.0 + player_size_by2;
        let y_max = window.height() - player_size_by2;

        let mut translation = transform.translation;

        //bounds the players x coordinates to prevent the player going out of bounds(off screen)
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        //bounds the players y coordinates to prevent the player going out of bounds(off screen)
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
        player.position = Vec2::new(translation.x, translation.y);
    }
}

pub fn yep_move_enemy(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let enemy_direction = Vec3::new(enemy.enemy_direction.x, enemy.enemy_direction.y, 0.0);
        transform.translation += enemy_direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn dont_stick_to_walls_challenge(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let enemy_size_by2 = ENEMY_SIZE / 2.0; //32.0
    let x_min = 0.0 + enemy_size_by2;
    let x_max = window.width() - enemy_size_by2;
    let y_min = 0.0 + enemy_size_by2;
    let y_max = window.height() - enemy_size_by2;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.enemy_direction.x *= -1.0;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.enemy_direction.y *= -1.0;
        }
    }
}

pub fn confine_enemy(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let enemy_size_by2 = ENEMY_SIZE / 2.0; //32.0
    let x_min = 0.0 + enemy_size_by2;
    let x_max = window.width() - enemy_size_by2;
    let y_min = 0.0 + enemy_size_by2;
    let y_max = window.height() - enemy_size_by2;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        //bound enemy x
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        //bound enemy y
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn collision_l(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("you have been hit L");
                //let sound_effect = asset_server.load("2augh.ogg");
                // audio.play(sound_effect);
                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn collision_2(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    tear_query: Query<&Transform, With<Tear>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    //despawn the enemy if it collides with a tear
    if let Ok((enemy_entity, enemy_transform)) = enemy_query.get_single_mut() {
        for tear_transform in tear_query.iter() {
            let distance = enemy_transform
                .translation
                .distance(tear_transform.translation);
            let enemy_radius = ENEMY_SIZE / 2.0;
            let tear_radius = TEAR_SIZE / 2.0;
            if distance < enemy_radius + tear_radius {
                println!("you have hit an enemy");
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}

pub fn yep_tear(
    mut player: Query<&mut Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut tear_direction = Vec2::new(0.0, 0.0);
    let mut spawn = false;

    if input.pressed(KeyCode::Up) {
        tear_direction += Vec2::new(0.0, 1.0);
        spawn = true;
    }
    if input.pressed(KeyCode::Down) {
        tear_direction += Vec2::new(0.0, -1.0);
        spawn = true;
    }
    if input.pressed(KeyCode::Left) {
        tear_direction += Vec2::new(-1.0, 0.0);
        spawn = true;
    }
    if input.pressed(KeyCode::Right) {
        tear_direction += Vec2::new(1.0, 0.0);
        spawn = true;
    }

    for mut player in &mut player {
        if player.tear_cooldown > 0.0 {
            player.tear_cooldown -= COOLDOWN_CONST * time.delta_seconds();
            player.tear_cooldown = player.tear_cooldown.clamp(0.0, 100.0);
            spawn = false;
        }

        tear_direction += player.direction * (PLAYER_SPEED / TEAR_SPEED);

        if tear_direction == Vec2::new(0.0, 0.0) {
            spawn = false;
        }

        if spawn == true {
            let texture_handle = asset_server.load("tear.png");

            let tear = Tear {
                direction: tear_direction,
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(25.0, 25.0)),
                        ..default()
                    },
                    texture: texture_handle,
                    transform: Transform {
                        translation: Vec3::new(player.position.x, player.position.y, 0.0),
                        ..default()
                    },
                    ..default()
                },
                tear,
            ));

            player.tear_cooldown = COOLDOWN_CONST * 0.2;
        }
    }
}

pub fn yep_move_tear(mut tears: Query<(&mut Transform, &Tear)>, time: Res<Time>) {
    for (mut transform, tear) in &mut tears {
        transform.translation.x += tear.direction.x * TEAR_SPEED * time.delta_seconds();
        transform.translation.y += tear.direction.y * TEAR_SPEED * time.delta_seconds();
    }
}
