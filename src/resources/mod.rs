use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::*;
use crossterm::event::KeyEvent as CtKeyEvent;

use crate::components::{Position, Tile};

#[derive(Resource, Deref, DerefMut)]
pub struct PlayerResource(pub Position);

impl Tile for PlayerResource {
    const CHAR: char = '@';
}

#[derive(Resource, Default, Deref)]
pub struct Score(pub i32);

#[derive(Resource, Deref, DerefMut)]
pub struct Stdout(pub std::io::Stdout);

impl Default for Stdout {
    fn default() -> Self {
        Self(std::io::stdout())
    }
}

#[derive(Resource, Deref)]
pub struct KeyEvent(pub CtKeyEvent);

#[derive(Resource, Deref, DerefMut)]
pub struct UseNumpad(pub bool);

#[derive(Resource, Deref, DerefMut)]
pub struct FullRedrawRequired(pub bool);

#[derive(Resource)]
pub enum Scene {
    Init,
    Playing,
    GameOver,
    Quitting,
}
