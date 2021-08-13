pub mod util;
pub mod state;

use util::events::{
    Event, 
    Events,
};
use util::utils::format_duration;
use state::app::{
    App, 
    TimerMode,
    MAX_FOCUS_DURATION,
    MAX_BREAK_DURATION
};

use std::{
    error::Error, 
    io,
    time::Duration,
};
use tui::{
    backend::TermionBackend,
    layout::{Layout, Constraint, Direction},
    style::{Style, Modifier, Color},
    widgets::{Block, Borders, Gauge, LineGauge, Paragraph},
    text::{Span, Spans},
    Terminal
};
use termion::{event::Key, raw::IntoRawMode};

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut app = App::new();
    
    terminal.clear();
    loop {
        terminal.draw(|rect| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(5),
                        Constraint::Length(3),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ].as_ref()
                )
                .split(rect.size());

            let timer_guage = draw_timer(&mut app);
            rect.render_widget(timer_guage, chunks[0]);

            if app.is_editing_duration() {
                let duration = draw_duration(&app.duration, &app.edit_mode);
                rect.render_widget(duration, chunks[1])
            }
            let text = vec![
                Spans::from(Span::raw(format!("Ratio: {}\n", app.ratio()))),
                Spans::from(Span::raw(format!("Remaining Time: {}\n", app.time_remaining.as_secs()))),
                Spans::from(Span::raw(format!("Remaining Ticks: {}\n", app.ticks_remaining))),
                Spans::from(Span::raw(format!("Focus Time: {}\n", app.focus_time.as_secs()))),
                Spans::from(Span::raw(format!("Break Time: {}\n", app.break_time.as_secs()))),
                Spans::from(Span::raw(format!("Running: {}\n", app.running))),
            ];
            let paragraph = Paragraph::new(text)
                .block(
                    Block::default()
                        .title("Settings")
                        .borders(Borders::ALL)
                );

            rect.render_widget(paragraph, chunks[2]);
        })?;

        match events.next()? {
            Event::Input(input) => {
                if app.is_editing_duration() {
                    if input == Key::Char('s') {
                        app.set_duration();
                    }
                    if input == Key::Char(']') {
                        app.increase_duration();
                    }
                    if input == Key::Char('[') {
                        app.decrease_duration();
                    }
                }
                if input == Key::Char('q') {
                    terminal.clear();
                    break;
                }
                if input == Key::Char('s') {
                    app.start_timer();
                }
                if input == Key::Char('b') {
                    app.edit_duration(TimerMode::Break);
                }
                if input == Key::Char('f') {
                    app.edit_duration(TimerMode::Focus);
                }
                if input == Key::Char('r') {
                    app.reset_duration();
                }
                if input == Key::Char('x') {
                    app.switch_timer_mode();
                }
            }
            Event::Tick => {
                app.update();
            }
        }
    }
    Ok(())
}

fn draw_duration<'a>(duration: &'a Duration, mode: &'a TimerMode) -> LineGauge<'a> {
    let label = format_duration(duration);
    let (divisor, color, title) = match mode {
        TimerMode::Break => {
            (MAX_BREAK_DURATION.as_secs(), Color::Cyan, "Break Duration")
        }
        TimerMode::Focus => {
            (MAX_FOCUS_DURATION.as_secs(), Color::Green, "Focus Duration")
        }
    };
    let ratio = duration.as_secs() as f64 / divisor as f64;
    LineGauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title),
        )
        .gauge_style(
            Style::default()
                .fg(color)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .label(label)
        .ratio(ratio)
}

fn draw_timer<'a>(app: &'a mut App) -> Gauge<'a> {
    let label = format_duration(&app.time_remaining) + " remaining";
    let (color, title) = match app.timer_mode {
        TimerMode::Break => (Color::Blue, "Break Time Remaining"),
        TimerMode::Focus => (Color::Green, "Focus Time Remaining"),
    };
    let ratio = app.ratio();
    let timer_guage = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
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
        )
        .gauge_style(
            Style::default()
                .fg(color) // Full bar color
                .bg(Color::Black) // Empty bar color
        );
    return timer_guage;
}
