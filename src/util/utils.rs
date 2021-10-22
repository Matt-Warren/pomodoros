use std::time::Duration;

pub fn format_duration(dur: &Duration) -> String {
    let min_left = dur.as_secs() / 60;
    let sec_left = dur.as_secs() % 60;
    return format!("{}:{:0>2}", min_left, sec_left)
}
