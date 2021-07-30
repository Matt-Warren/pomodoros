pub mod util;

use util::events::{Event, Events};

use std::{error::Error, io};
use tui::{
    backend::TermionBackend,
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders},
    Terminal
};
use termion::{event::Key, raw::IntoRawMode};

struct App {
    progress1: u16,
}

impl App {
    fn new() -> App {
        App {
            progress1: 0,
        }
    }

    fn update(&mut self) {
        self.progress1 += 5;
        if self.progress1 > 1000 {
            self.progress1 = 0;
        }
    }
}

fn main()  -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut app = App::new();
    
    terminal.clear();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ].as_ref()
                )
                .split(f.size());
            let block = Block::default()
                .title("Block")
                .borders(Borders::ALL);
            f.render_widget(block, chunks[0]);
            let block = Block::default()
                .title("Block 2")
                .borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        })?;

        match events.next()? {
            Event::Input(input) => {
                if input == Key::Char('q') {
                    break;
                }
            }
            Event::Tick => {
                app.update();
            }
        }
    }
    Ok(())
}
