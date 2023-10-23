mod keys;
use std::io::{self, Stdout, Write};
use std::iter;
use std::{cmp::min, fmt::Debug};

use crossterm::{cursor, style};
use crossterm::{event::Event, terminal, QueueableCommand};
use rand::Rng;

pub use keys::{Key, KeyBindings, MoveKey};
const COLS: isize = 60;
const ROWS: isize = 22;

// we're just gonna hope and pretend my math is Flawless
// and that I can do `x as usize` Whenever i fucking Want
type Coord = (isize, isize);

#[derive(Debug)]
pub struct State {
    keybindings: KeyBindings,
    rng: rand::prelude::ThreadRng,

    /// Current level.
    level: u16,
    /// A limit on `level`, determining how many robots are permitted to spawn.
    max_level: u16,
    /// Only God knows how this is to be calculated
    #[allow(dead_code)]
    score: u32,

    player: Coord,
    robots: Vec<Coord>,
    heaps: Vec<Coord>,
}

impl State {
    fn rand_coordinate(&mut self) -> Coord {
        (self.rng.gen_range(0..COLS), self.rng.gen_range(0..ROWS))
    }

    pub fn new_game(keybindings: KeyBindings, max_level: u16) -> Self {
        Self {
            keybindings,
            rng: rand::thread_rng(),
            level: 0,
            max_level,
            score: 0,
            player: (0, 0),
            robots: vec![],
            heaps: vec![],
        }
    }

    /// Starts the next level.
    pub fn next_level(&mut self) {
        self.level += 1;

        let robot_total: usize = (min(self.level, self.max_level) * 10).into();

        let mut robots: Vec<Coord> = Vec::with_capacity(robot_total);
        let heaps: Vec<Coord> = Vec::with_capacity(robot_total / 2);

        // Spawn player
        let player = self.rand_coordinate();

        // Spawn robots
        for _ in 0..robot_total {
            let mut coord = self.rand_coordinate();
            while coord == player || robots.contains(&coord) {
                coord = self.rand_coordinate();
            }

            robots.push(coord);
        }

        // Update state
        self.player = player;
        self.robots = robots;
        self.heaps = heaps;
    }

    /// Returns the current keybinding for a key
    pub fn parse_event(&self, event: Event) -> Option<keys::Key> {
        self.keybindings.get(event)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new_game(KeyBindings::default(), 4)
    }
}

// This impl block is for rendering to the terminal.
impl State {
    /// Queues commands to stdout that clear the terminal and write a box's border.
    fn render_box_exterior(&self, stdout: &mut Stdout) -> io::Result<()> {
        stdout.queue(terminal::EnterAlternateScreen)?;
        stdout.queue(cursor::MoveTo(0, 0))?;

        {
            let dashes = "─".repeat(COLS as usize);

            stdout.queue(style::Print('┌'))?;
            stdout.queue(style::Print(&dashes))?;
            stdout.queue(style::Print('┐'))?;
            stdout.queue(cursor::MoveToNextLine(1))?;

            let right_col: u16 = (COLS + 1).try_into().unwrap();
            for _ in 0..ROWS {
                stdout.queue(style::Print('│'))?;
                stdout.queue(cursor::MoveToColumn(right_col))?;
                stdout.queue(style::Print('│'))?;
                stdout.queue(cursor::MoveToNextLine(1))?;
            }

            stdout.queue(style::Print('└'))?;
            stdout.queue(style::Print(&dashes))?;
            stdout.queue(style::Print('┘'))?;
        }

        Ok(())
    }

    /// Queues commands to stdout to render current grid
    fn render_box_interior(&self, stdout: &mut Stdout) -> io::Result<()> {
        // render self.grid
        macro_rules! render_coord {
            ($coord:expr, $ch:literal) => {
                stdout.queue(cursor::MoveTo($coord.0 as u16 + 1, $coord.1 as u16 + 1))?;
                stdout.queue(style::Print($ch))?;
            };
        }

        render_coord!(self.player, '+');

        for heap in &self.heaps {
            render_coord!(heap, '*');
        }

        for robot in &self.robots {
            render_coord!(robot, 'R');
        }

        Ok(())
    }

    /// Queues stdout commands to clear current grid
    pub fn clear_box_interior(&self, stdout: &mut Stdout) -> io::Result<()> {
        let coords = iter::once(&self.player)
            .chain(self.robots.iter())
            .chain(self.heaps.iter());
        for coord in coords {
            stdout.queue(cursor::MoveTo(coord.0 as u16, coord.1 as u16))?;
            stdout.queue(style::Print(' '))?;
        }
        Ok(())
    }

    pub fn render_side_panel(&self, stdout: &mut Stdout) -> io::Result<()> {
        let messages = [
            "Full sidebar",
            "not yet",
            "implemented!",
            "",
            "Score:",
            &*(format!("{}", self.score)),
        ];
        const COL: u16 = (COLS + 3) as u16;

        stdout.queue(cursor::MoveTo(0, 1))?;
        for m in messages {
            stdout.queue(cursor::MoveToColumn(COL))?;
            stdout.queue(style::Print(m))?;
            stdout.queue(cursor::MoveToNextLine(1))?;
        }

        Ok(())
    }

    /// Rerenders the grid and side panel.
    pub fn render_full(&self, stdout: &mut Stdout) -> io::Result<()> {
        self.render_box_interior(stdout)?;
        self.render_side_panel(stdout)?;
        stdout.flush()
    }

    /// Renders the box exterior & side panel, and checks that the terminal is properly sized.
    pub fn render_initial(&self, stdout: &mut Stdout) -> io::Result<()> {
        let min_size = (COLS + 12, ROWS + 2);

        let screen = terminal::size().expect("there should be a terminal");
        let screen = (
            isize::try_from(screen.0).unwrap(),
            isize::try_from(screen.1).unwrap(),
        );

        if screen.0 < min_size.0 || screen.1 < min_size.1 {
            panic!(
                "terminal must be at least {}x{} large! got {}x{}",
                min_size.0, min_size.1, screen.0, screen.1
            );
        }
        self.render_box_exterior(stdout)?;
        self.render_side_panel(stdout)?;
        stdout.flush()
    }
}
