use bevy_ecs::prelude::*;

/// 0-indexed grid positions.
#[derive(Component)]
pub struct Position {
    pub col: u16,
    pub row: u16,
}

pub trait Tile {
    const CHAR: char;
}

//#[derive(Component)]
//pub struct PlayerComponent;
//
//impl Tile for PlayerComponent {
//    const CHAR: char = '@';
//}

#[derive(Component)]
pub struct Enemy;

impl Tile for Enemy {
    const CHAR: char = '+';
}

#[derive(Component)]
pub struct JunkPile;

impl Tile for JunkPile {
    const CHAR: char = '*';
}
