use crate::{grid, Stdout};
use bevy_ecs::prelude::*;
use crossterm::{execute, terminal};

pub fn initial_checks(mut stdout: ResMut<Stdout>) {
    let (cols, rows) = match crossterm::terminal::size() {
        Ok(size) => size,
        Err(e) => {
            panic!("error: while reading screen size: {:?}", e);
        }
    };

    if cols < grid::MIN_COLS || rows < grid::MIN_ROWS {
        panic!("error: need at least a 24x80 screen");
    }

    // TODO: alternate screen, cleanup function
    execute!(stdout, terminal::Clear(terminal::ClearType::All))
        .expect("stdout should be available");
}
