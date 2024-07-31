use std::io::Write as _;

use bevy_ecs::prelude::*;
use crossterm::queue;
use crossterm::{cursor, style::Print, terminal};

use crate::components::Tile as _;
use crate::grid;
use crate::resources::{FullRedrawRequired, PlayerResource, Stdout};

pub fn render(
    mut stdout: ResMut<Stdout>,
    player: Res<PlayerResource>,
    mut redraw: ResMut<FullRedrawRequired>,
) {
    macro_rules! equeue {
        ($command:expr) => {
            queue!(stdout, $command).expect("stdout should be available");
        };
    }

    'redraw: {
        if **redraw {
            equeue!(terminal::Clear(terminal::ClearType::All));
            equeue!(cursor::MoveTo(
                grid::OUTER_GRID_LEFT_COLUMN,
                grid::OUTER_GRID_TOP_ROW
            ));
            equeue!(Print(&*grid::ASCII_TOP_ROW));
            for _ in 0..grid::INNER_GRID_ROW_TOTAL {
                equeue!(Print(&*grid::ASCII_MIDDLE_ROW));
            }
            equeue!(Print(&*grid::ASCII_BOTTOM_ROW));
        } else if player.is_changed() {
            todo!("clearing inner grid")
        } else {
            break 'redraw;
        }

        equeue!(cursor::MoveTo(
            player.col + grid::INNER_GRID_LEFT_COLUMN,
            player.row + grid::INNER_GRID_TOP_ROW
        ));
        equeue!(Print(PlayerResource::CHAR));
        equeue!(cursor::MoveTo(
            player.col + grid::INNER_GRID_LEFT_COLUMN,
            player.row + grid::INNER_GRID_TOP_ROW,
        ));

        stdout.flush().unwrap();
    }

    **redraw = false;
}
