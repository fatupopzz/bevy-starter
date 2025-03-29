use bevy::prelude::*;

use crate::{
    components::*,
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    utils::random_pipe_position,
};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawn the 2D camera
    commands.spawn(Camera2dBundle::default());

    // Spawn the background
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/background.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(WINDOW_WIDTH + 288.0 * 2., WINDOW_HEIGHT)), // la imagen goes brrr

                ..default()
            },
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: false,
            stretch_value: 1.,
        },
        Background,
    ));

    // Spawn the ground
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/base.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(WINDOW_WIDTH + 288. * 2., 112.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -250., 1.),
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: false,
            stretch_value: 1.,
        },
        Ground,
    ));

    // Spawn the game over text
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/game-over.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            visibility: Visibility::Hidden,
            ..default()
        },
        GameOverText,
    ));

    // Spawn del texto de presiona espacio
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/space.png"),
            transform: Transform::from_xyz(0.0, -50.0, 1.0),
            ..default()
        },
        PressSpaceBarText(Timer::from_seconds(0.5, TimerMode::Repeating)),
    ));

    let number_layout: TextureAtlasLayout =
        TextureAtlasLayout::from_grid(UVec2::new(24, 36), 1, 10, None, None);
    let number_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(number_layout);

    for i in 0..3 {
        let starting_point = -350. + (i as f32 * (24. * 2.));

        // Spawn del texto de score
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("texture/numbers.png"),
                transform: Transform::from_xyz(starting_point, 200., 1.),
                ..default()
            },
            TextureAtlas {
                index: 0,
                layout: number_texture_atlas_layout.clone(),
            },
            ScoreText,
        
        ));
    }

        // Spawn el pajaro
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("texture/bird.png"),
                transform: Transform::from_xyz(0., 0., 2.),
                ..default()
            },
            TextureAtlas {
                index: 1,
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    UVec2::new(34, 24),
                    3,
                    1,
                    None,
                    None,
                )),
            },
            Bird {
                timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                velocity: 0.,
            },
        ));

// Spawn los tubos
for i in 0..5 {
    let delta_x = i as f32 * 200.; // Space between pairs of pipes
    let (lower_y, upper_y) = random_pipe_position();
    let mut transform = Transform::from_xyz(350. + delta_x, lower_y, 0.5);

    // El tubo de abajo
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/pipe.png"),
            transform,
            ..default()
        },
        LowerPipe,
    ));

    // Rotacion de 180 grados
    transform.rotate(Quat::from_rotation_z(std::f32::consts::PI));
    // Cambiar la posicion de la tuberia
    transform.translation.y = upper_y;

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/pipe.png"),
            transform,
            ..default()
        },
        UpperPipe { passed: false },
    ));
}
}