use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::fmt::Debug;

#[derive(Debug)]
pub enum MoveKey {
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownLeft,
    Down,
    DownRight,
}

impl MoveKey {
    pub fn direction(&self) -> (isize, isize) {
        match self {
            Self::UpLeft => (-1, -1),
            Self::Up => (0, -1),
            Self::UpRight => (1, -1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::DownLeft => (-1, 1),
            Self::Down => (0, 1),
            Self::DownRight => (1, 1),
        }
    }
}

#[derive(Debug)]
pub enum Key {
    Move(MoveKey),
    Teleport,
    WaitForEnd,
    Quit,
}

impl From<MoveKey> for Key {
    fn from(value: MoveKey) -> Self {
        Self::Move(value)
    }
}

pub struct KeyBindings(Box<dyn Fn(Event) -> Option<Key>>);
impl KeyBindings {
    pub fn get(&self, event: Event) -> Option<Key> {
        self.0(event)
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        let f = |e| {
            if let Event::Key(k) = e {
                use KeyCode::Char;
                use MoveKey::*;
                let key: Key = match k.code {
                    Char('7') => UpLeft.into(),
                    Char('8') => Up.into(),
                    Char('9') => UpRight.into(),

                    Char('4') => Left.into(),
                    Char('6') => Right.into(),

                    Char('1') => DownLeft.into(),
                    Char('2') => Down.into(),
                    Char('3') => DownRight.into(),

                    Char('5') => Key::Teleport,
                    Char('.' | 'q') => Key::Quit,
                    Char('c') if k.modifiers == KeyModifiers::CONTROL => Key::Quit,
                    KeyCode::Enter => Key::WaitForEnd,
                    _ => return None,
                };
                Some(key)
            } else {
                None
            }
        };
        Self(Box::new(f))
    }
}

impl Debug for KeyBindings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("KeyBinding()").finish()
    }
}
