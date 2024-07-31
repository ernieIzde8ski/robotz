use std::io;
use std::process::ExitCode;

use bevy_ecs::prelude as bevy_ecs;
use bevy_ecs::*;

pub mod components;
pub mod grid;
pub mod resources;
pub mod systems;

use resources::*;

fn read_key_event() -> io::Result<crossterm::event::KeyEvent> {
    loop {
        match crossterm::event::read()? {
            crossterm::event::Event::Key(e) => return Ok(e),
            _ => continue,
        }
    }
}

fn main() -> ExitCode {
    let mut world = World::default();
    world.insert_resource(Scene::Init);

    let init_system = world.register_system(systems::initial_checks);
    let mut gameplay_loop = Schedule::default();
    gameplay_loop.add_systems((systems::run_turn, systems::render).chain());

    '_init: {
        world.insert_resource(Stdout::default());
        world.run_system(init_system).unwrap();
    }

    'round: loop {
        match world
            .get_resource::<Scene>()
            .expect("should be initialized")
        {
            // Setup state for gameplay
            Scene::Init => {
                world.remove_resource::<KeyEvent>();
                world.insert_resource(PlayerResource(components::Position {
                    col: grid::INNER_GRID_COLUMN_TOTAL / 2,
                    row: grid::INNER_GRID_ROW_TOTAL / 2,
                }));
                world.insert_resource(UseNumpad(false));
                world.insert_resource(FullRedrawRequired(true));
                world.insert_resource(Scene::Playing);
            }

            // Run the gameplay loop and wait for a keypress
            Scene::Playing => {
                gameplay_loop.run(&mut world);
                match read_key_event() {
                    Ok(ev) => world.insert_resource(KeyEvent(ev)),
                    Err(err) => {
                        eprintln!("error: couldn't read key event: {}", err);
                        return 1.into();
                    }
                }
            }

            Scene::GameOver => todo!(),
            Scene::Quitting => break 'round,
        }
    }

    0.into()
}
