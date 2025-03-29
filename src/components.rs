//componentes jejejej
use bevy::prelude::*;


//el fondo de pantalla
#[derive(Component)]
pub struct Background;


//el suelo
#[derive(Component)]
pub struct Ground;

//texto del gameover
#[derive(Component)]
pub struct GameOverText;

//el press space
#[derive(Component)]
pub struct PressSpaceBarText(pub Timer);


//el scoretext
#[derive(Component)]
pub struct ScoreText;


//los tubos 

#[derive(Component)]
pub struct LowerPipe;

//el pajarito que se pajea pipipi
#[derive(Component)]
pub struct Bird {
    pub timer: Timer,
    pub velocity: f32,
}

//el coso de las cosas
#[derive(Component)]
pub struct UpperPipe{
    pub passed: bool,
}

