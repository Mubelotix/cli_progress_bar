use crate::{
    pb::ProgressBar,
    style::{Color, Mode, Style},
};
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref CURRENT_PROGRESS_BAR: Mutex<Option<ProgressBar>> = Mutex::new(None);
}

pub fn has_progress_bar() -> bool {
    CURRENT_PROGRESS_BAR.lock().unwrap().is_some()
}

pub fn set_progress_bar(progress_bar: ProgressBar) {
    *CURRENT_PROGRESS_BAR.lock().unwrap() = Some(progress_bar);
}

pub fn init_progress_bar(max: usize) {
    let progress_bar = ProgressBar::new(max);
    set_progress_bar(progress_bar);
}

pub fn set_progress_bar_progression(progression: usize) {
    match *CURRENT_PROGRESS_BAR.lock().unwrap() {
        Some(ref mut progress_bar) => progress_bar.set_progression(progression),
        None => eprintln!("ERROR: Unable to set progress bar progression (no progress bar)"),
    }
}

pub fn inc_progress_bar() {
    match *CURRENT_PROGRESS_BAR.lock().unwrap() {
        Some(ref mut progress_bar) => progress_bar.inc(),
        None => eprintln!("ERROR: Unable to increase progress bar progression (no progress bar)"),
    }
}

pub fn set_progress_bar_width(width: usize) {
    match *CURRENT_PROGRESS_BAR.lock().unwrap() {
        Some(ref mut progress_bar) => progress_bar.set_width(width),
        None => eprintln!("ERROR: Unable to set progress bar width (no progress bar)"),
    }
}

pub fn print_progress_bar_info(info_name: &str, text: &str, info_color: Color, info_style: Style) {
    match *CURRENT_PROGRESS_BAR.lock().unwrap() {
        Some(ref mut progress_bar) => {
            progress_bar.print_info(info_name, text, info_color, info_style)
        }
        None => eprintln!("ERROR: Unable to print progress bar info (no progress bar)"),
    }
}

pub fn set_progress_bar_action(action: &str, color: Color, style: Style) {
    match *CURRENT_PROGRESS_BAR.lock().unwrap() {
        Some(ref mut progress_bar) => progress_bar.set_action(action, color, style),
        None => eprintln!("ERROR: Unable to set progress bar action (no progress bar)"),
    }
}

pub fn set_progress_bar_action_with_mode(action: &str, color: Color, style: Style, mode: Mode) {
    match *CURRENT_PROGRESS_BAR.lock().unwrap() {
        Some(ref mut progress_bar) => progress_bar.set_action_with_mode(action, color, style, mode),
        None => eprintln!("ERROR: Unable to set progress bar action (no progress bar)"),
    }
}

pub fn finalize_progress_bar() {
    match *CURRENT_PROGRESS_BAR.lock().unwrap() {
        Some(ref mut progress_bar) => progress_bar.finalize(),
        None => eprintln!("ERROR: Unable to finalize progress bar (no progress bar)"),
    }
}
