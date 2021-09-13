use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{time::Duration};
use serde_json::{Result, Value};
use serde::{Serialize, Deserialize};
// looking into https://github.com/sede-json

pub const SETTINGS_FILE_PATH: &str = "../settings.json";

#[derive(Serialize, Deserialize)]
pub struct Settings {
    focus_time: Duration,
    break_time: Duration,
}

impl Settings {
    pub fn load() -> Settings {
        let path = Path::new(SETTINGS_FILE_PATH);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}"),
            Ok(file) => file,
        }

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("file loaded"),
        }
        Settings {
            focus_time: Duration::from_secs(60),
            break_time: Duration::from_secs(5),
        }
    }

    pub fn save_settings() -> () {

    }
}
