use bevy::prelude::*;

// es el plugin que se encarga de manejar las ventanas

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Piggy".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }));
    }

}