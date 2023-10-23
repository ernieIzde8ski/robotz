use std::borrow::BorrowMut;

pub mod state;
use crossterm::{cursor, QueueableCommand};
use crossterm::{event::read, terminal};
use state::{Key, State};
use std::io::{self, Stdout, Write};

use std::panic;

fn game_start(stdout: &mut Stdout) -> io::Result<()> {
    stdout.queue(terminal::EnterAlternateScreen)?;
    stdout.queue(cursor::Hide)?;
    stdout.flush()?;
    terminal::enable_raw_mode()
}

fn game_loop(stdout: &mut Stdout) -> io::Result<()> {
    let mut game_state = State::default();

    game_state.next_level();
    game_state.render_initial(stdout)?;

    loop {
        game_state.render_full(stdout)?;
        let key = game_state.parse_event(read()?);
        game_state.clear_box_interior(stdout)?;
        match key {
            Some(Key::Move(_)) => todo!("movement"),
            Some(Key::WaitForEnd) => todo!("wait"),
            Some(Key::Teleport) => todo!("teleportation"),
            Some(Key::Quit) => {
                let confirmation = game_state.parse_event(read()?);
                if matches!(confirmation, Some(Key::Quit)) {
                    break;
                }
            }
            None => continue,
        };
    }

    Ok(())
}

fn game_end(stdout: &mut Stdout) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    stdout.queue(cursor::Show)?;
    stdout.queue(terminal::LeaveAlternateScreen)?;
    stdout.flush()
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    let default_panic_hook = panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        game_end(io::stdout().borrow_mut()).unwrap();
        default_panic_hook(info)
    }));

    game_start(&mut stdout)?;
    let result = game_loop(&mut stdout);
    game_end(&mut stdout)?;
    result
}
