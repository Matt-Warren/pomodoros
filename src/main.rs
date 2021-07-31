pub mod util;

use util::events::{Event, Events};

use std::{error::Error, io};
use tui::{
    backend::TermionBackend,
    layout::{Layout, Constraint, Direction},
    style::{Style, Modifier, Color},
    widgets::{Block, Borders, Gauge, Paragraph},
    text::{Span, Spans},
    Terminal
};
use termion::{event::Key, raw::IntoRawMode};


enum TimerMode {
    Focus,
    Break,
}

struct App {
    focus_time: u16,
    break_time: u16,
    time_remaining: u16,
    ticks_remaining: u16,
    timer_mode: TimerMode,
    running: bool,
}

impl App {
    fn new() -> App {
        App {
            focus_time: 60,
            break_time: 5*60, // default to 5 min
            time_remaining: 0,
            ticks_remaining: 0,
            timer_mode: TimerMode::Focus,
            running: false,
        }
    }

    fn update(&mut self) {
        if self.running {
            if self.ticks_remaining == 0 {
                self.ticks_remaining = 0;
                self.running = false;
                return
            }
            self.ticks_remaining -= 1;

            self.time_remaining = self.ticks_remaining / 5
        }
    }

    fn ratio(&mut self) -> f64 {
        if self.time_remaining == 0 {
            return 0.0;
        }
        let x;
        match self.timer_mode {
            TimerMode::Focus => {
                if self.focus_time == 0 {
                    return 0.0
                }
                x = (self.ticks_remaining) as f64 / (self.focus_time * 5) as f64;
            }
            TimerMode::Break => {
                if self.break_time == 0 {
                    return 0.0
                }
                x = (self.ticks_remaining) as f64 / (self.break_time * 5) as f64;
            }
        }
        return x;
    }

    fn start_timer(&mut self) -> () {
        self.running = true;
    }
    
    
    fn set_duration(&mut self) -> () {
        self.time_remaining = 60;
        self.ticks_remaining = 60 * 5;
    }
    
    
    fn format_label(&mut self) -> String {
        let min_left = self.time_remaining / 60;
        let sec_left = self.time_remaining % 60;
        return format!("{}:{:0>2} remaining", min_left, sec_left)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
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

            let label = app.format_label();
            let ratio = app.ratio();
            let timer_guage = Gauge::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Time Remaining")
                        .style(
                            Style::default()
                                .bg(Color::Black)
                                .fg(Color::White)
                        )
                )
                .ratio(ratio)
                .label(label)
                .style(
                    Style::default()
                        .fg(Color::Red)
                        .bg(Color::Green)
                        // .add_modifier(Modifier::BOLD)
                )
                .gauge_style(
                    Style::default()
                        .fg(Color::LightGreen) // Full bar color
                        .bg(Color::Black) // Empty bar color
                );
            f.render_widget(timer_guage, chunks[0]);

            let text = vec![
                Spans::from(Span::raw(format!("Ratio: {}\n", app.ratio()))),
                Spans::from(Span::raw(format!("Remaining Time: {}\n", app.time_remaining))),
                Spans::from(Span::raw(format!("Remaining Ticks: {}\n", app.ticks_remaining))),
                Spans::from(Span::raw(format!("Focus Time: {}\n", app.focus_time))),
                Spans::from(Span::raw(format!("Break Time: {}\n", app.break_time))),
                Spans::from(Span::raw(format!("Running: {}\n", app.running))),
            ];
            let paragraph = Paragraph::new(text)
                .block(
                    Block::default()
                        .title("Settings")
                        .borders(Borders::ALL)
                );

            f.render_widget(paragraph, chunks[1]);
        })?;

        match events.next()? {
            Event::Input(input) => {
                if input == Key::Char('q') {
                    break;
                }
                if input == Key::Char('s') {
                    app.start_timer();
                }
                if input == Key::Char('d') {
                    app.set_duration();
                }
            }
            Event::Tick => {
                app.update();
            }
        }
    }
    Ok(())
}


