use bevy::prelude::*;


#[derive(Resource, Default)]
pub struct Game {
    pub score: u32,
    pub state: GameState,
}
// el estado del juego ehhrbefha

#[derive(PartialEq)]
pub enum GameState {
    Active,
    Inactive,
    GameOver,
}
//------------------------------------------------------------------------------------------------
impl Default for GameState {
    fn default() -> Self {
        GameState::Inactive
    }
}

    // funciones para saber si el juego esta activo o no
    pub fn is_game_active(game: Res<Game>) -> bool {
        game.state == GameState::Active
    }
//------------------------------------------------------------------------------------------------
    pub fn is_game_not_active(game: Res<Game>) -> bool {
        game.state != GameState::Active
    }
