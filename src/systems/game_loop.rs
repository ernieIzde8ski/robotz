use bevy_ecs::prelude::*;
use crossterm::event::{KeyCode, KeyModifiers};

use crate::resources::{KeyEvent, PlayerResource, UseNumpad};

pub fn run_turn(
    _player: ResMut<PlayerResource>,
    key_event: Option<Res<KeyEvent>>,
    use_numpad: Res<UseNumpad>,
) {
    if let Some(ke) = key_event {
        //let shift_pressed = ke.modifiers.contains(KeyModifiers::SHIFT);
        let ctrl_pressed = ke.modifiers.contains(KeyModifiers::CONTROL);

        match ke.code {
            KeyCode::Char('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9')
                if !**use_numpad =>
            {
                todo!("counts")
            }
            KeyCode::Left | KeyCode::Char('h' | '4') => todo!("keypresses"),
            KeyCode::Down | KeyCode::Char('j' | '2') => todo!("keypresses"),
            KeyCode::Up | KeyCode::Char('k' | '8') => todo!("keypresses"),
            KeyCode::Right | KeyCode::Char('l' | '6') => todo!("keypresses"),
            KeyCode::Char('y' | '7') => todo!("diagonal keypresses"),
            KeyCode::Char('u' | '9') => todo!("diagonal keypresses"),
            KeyCode::Char('b' | '1') => todo!("diagonal keypresses"),
            KeyCode::Char('n' | '3') => todo!("diagonal keypresses"),
            KeyCode::Char('.' | ' ') => todo!("nul ops"),
            KeyCode::Char('>') => todo!("nul ops"),
            KeyCode::Char('t') => todo!("teleports"),
            KeyCode::F(5) | KeyCode::Char('r') => todo!("manual screen refresh"),
            KeyCode::Char('w') => todo!("waiting"),
            KeyCode::Char('c') if ctrl_pressed => todo!("ctrl-c quitting"),
            KeyCode::Char('q') => todo!("q-key quitting"),
            _ => (),
        }
    }
}
