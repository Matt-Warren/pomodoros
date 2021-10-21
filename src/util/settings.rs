use std::fs::File;
use std::path::Path;
use serde_json;
use serde::{Serialize, Deserialize};
use std::io::BufReader;
// looking into https://github.com/serde-json

pub const SETTINGS_FILE_PATH: &str = "settings.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub focus_time: u64,
    pub break_time: u64,
}

const DEFAULT_SETTINGS : Settings = Settings {
    focus_time: 5,
    break_time: 5,
};

impl Settings {
    pub fn load() -> (Settings, String) {
        let path = Path::new(SETTINGS_FILE_PATH);
        let file = match File::open(&path) {
            Err(why) => {
                println!("couldn't open file at '{}'\nReason: {}\n using default settings and creating save file", SETTINGS_FILE_PATH, why);
                panic!("psyche no file created yet.");
            }
            Ok(file) => file,
        };

        let reader = BufReader::new(file);
        let wrapped_settings : std::result::Result<Settings, serde_json::Error> = serde_json::from_reader(reader);
        match wrapped_settings.is_err() {
            true => {
                return (DEFAULT_SETTINGS, wrapped_settings.unwrap_err().to_string())
            },
            false => {
                return (wrapped_settings.unwrap(), String::new())
            },
        }
    }

    pub fn save_settings(new_settings: Settings) -> () {

    }
}
