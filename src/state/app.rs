use std::{time::Duration};

use crate::util::settings;

pub const FACTOR: u64 = 60;
pub const MAX_FOCUS_DURATION: Duration = Duration::from_secs(FACTOR*60);
pub const MIN_FOCUS_DURATION: Duration = Duration::from_secs(FACTOR*10);
pub const MAX_BREAK_DURATION: Duration = Duration::from_secs(FACTOR*60);
pub const MIN_BREAK_DURATION: Duration = Duration::from_secs(FACTOR);

#[derive(PartialEq)]
pub enum TimerMode {
    Focus,
    Break,
}

#[derive(PartialEq)]
pub enum KeybindMode {
    TimerControl,
    Editing,
}

pub struct App {
    pub current_max: Duration,
    pub focus_time: Duration,
    pub break_time: Duration,
    pub time_remaining: Duration,
    pub ticks_remaining: u64,
    pub timer_mode: TimerMode,
    pub edit_mode: TimerMode,
    pub keybind_mode: KeybindMode,
    pub running: bool,
    pub editing_focus: bool,
    pub editing_break: bool,
    pub duration: Duration,
    pub messages: Vec<String>,
}

impl App {
    pub fn new(settings: settings::Settings) -> App {
        App {
            current_max: Duration::from_secs(60),
            focus_time: Duration::from_secs(settings.focus_time),
            break_time: Duration::from_secs(settings.break_time),
            time_remaining: Duration::from_secs(0),
            ticks_remaining: 0,
            timer_mode: TimerMode::Focus,
            edit_mode: TimerMode::Focus,
            keybind_mode: KeybindMode::TimerControl,
            running: false,
            editing_break: false,
            editing_focus: false,
            duration: Duration::from_secs(30),
            messages: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        if self.running {
            if self.ticks_remaining == 0 {
                self.ticks_remaining = 0;
                self.switch_timer_mode();
                return
            }
            self.ticks_remaining -= 1;

            self.time_remaining = Duration::from_secs(self.ticks_remaining / 5)
        }
    }

    pub fn ratio(& self) -> f64 {
        if self.time_remaining.as_secs() == 0  || self.current_max.as_secs() == 0 {
            return 0.0;
        }
        return (self.ticks_remaining) as f64 / (self.current_max.as_secs() * 5) as f64;   
    }

    pub fn start_timer(&mut self) -> () {
        self.running = true;
    }
    
    pub fn set_duration(&mut self) -> () {
        match self.edit_mode {
            TimerMode::Break => {
                self.break_time = self.duration;
            }
            TimerMode::Focus => {
                self.focus_time = self.duration;
            }
        }
        self.editing_break = false;
        self.editing_focus = false;
        self.keybind_mode = KeybindMode::TimerControl;
        if !self.running && (self.edit_mode == self.timer_mode) {
            self.current_max = self.duration;
        }
        self.update_settings();
    }

    pub fn edit_duration(&mut self, mode: TimerMode) -> () {
        match mode {
            TimerMode::Break => {
                self.editing_break = true;
                self.edit_mode = TimerMode::Break;
                self.duration = self.break_time;
            },
            TimerMode::Focus => {
                self.editing_focus = true;
                self.edit_mode = TimerMode::Focus;
                self.duration = self.focus_time;
            },
        };
        self.keybind_mode = KeybindMode::Editing;
    }

    pub fn reset_duration(&mut self) -> () {
        self.running = false;
        self.refill_timer();
    }

    pub fn toggle_running(&mut self) -> () {
        self.running = !self.running;
    }

    pub fn refill_timer(&mut self) -> () {
        self.time_remaining = match self.timer_mode {
            TimerMode::Break => {
                self.current_max = self.break_time;
                self.break_time
            }
            TimerMode::Focus => {
                self.current_max = self.focus_time;
                self.focus_time
            },
        };
        self.ticks_remaining = self.time_remaining.as_secs() * 5;
    }

    pub fn increase_duration(&mut self) -> () {
        let increase_amount = Duration::from_secs(FACTOR);
        match self.edit_mode {
            TimerMode::Break => {
                if self.duration + increase_amount >= MAX_BREAK_DURATION {
                    self.duration = MAX_BREAK_DURATION;
                    return
                }
            }
            TimerMode::Focus => {
                if self.duration + increase_amount >= MAX_FOCUS_DURATION {
                    self.duration = MAX_FOCUS_DURATION;
                    return
                }
            }
        }
        self.duration = self.duration + increase_amount;
    }

    pub fn decrease_duration(&mut self) -> () {
        match self.edit_mode {
            TimerMode::Break => {
                if self.duration <= MIN_BREAK_DURATION {
                    self.duration = MIN_BREAK_DURATION;
                    return
                }
            }
            TimerMode::Focus => {
                if self.duration <= MIN_FOCUS_DURATION {
                    self.duration = MIN_FOCUS_DURATION;
                    return
                }
            }
        }
        self.duration = self.duration - Duration::from_secs(FACTOR);
    }

    pub fn is_editing_duration(&mut self) -> bool {
        return self.editing_break || self.editing_focus
    }

    pub fn switch_timer_mode(&mut self) -> () {
        match self.timer_mode {
            TimerMode::Break => {
                self.timer_mode = TimerMode::Focus;
            }
            TimerMode::Focus => {
                self.timer_mode = TimerMode::Break;
            }
        }
        self.refill_timer();
    }

    fn update_settings(&mut self) -> () {
        let settings = settings::Settings::new(
            self.focus_time.as_secs(), 
            self.break_time.as_secs()
        );
        match settings::Settings::save_settings(settings) {
            Ok(()) => return,
            Err(err) => self.log(err.to_string())
        }
        
    }

    pub fn log(&mut self, message: String) -> () {
        self.messages.push(message);
    }
}
