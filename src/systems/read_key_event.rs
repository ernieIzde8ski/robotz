use bevy_ecs::prelude::*;

use crate::resources::KeyEvent;
use crate::Scene;

/// Reads for a keypress.
///
/// Uses blocking IO. Use at the **end** of a schedule.
pub fn read_key_event(world: &mut World) {
    loop {
        match crossterm::event::read() {
            Ok(crossterm::event::Event::Key(e)) => world.insert_resource(KeyEvent(e)),
            Ok(_) => continue,
            Err(_) => world.insert_resource(Scene::Quitting),
        }
        return;
    }
}
