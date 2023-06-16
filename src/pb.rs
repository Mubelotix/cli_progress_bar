use crate::style::*;
use std::io;
use std::io::Write;
use std::time::Instant;

pub struct ProgressBar {
    max: usize,
    progression: usize,
    width: usize,
    action: String,
    action_color: Color,
    action_style: Style,
    start: Option<Instant>,
}

impl ProgressBar {
    /// Creates a progress bar with the total number of actions.  
    /// You will need to call inc method when an action is completed and the bar progression will be incremented by 1.  
    /// Don't print things with print macro while the bar is running; use the print_info method instead.  
    /// 
    /// # Example
    /// 
    /// ```
    /// use progress_bar::progress_bar::ProgressBar;
    /// use progress_bar::color::{Color, Style};
    /// use std::{thread, time};
    /// 
    /// // if you have 81 pages to load
    /// let mut progress_bar = ProgressBar::new(81);
    /// progress_bar.set_action("Loading", Color::Blue, Style::Bold);
    ///
    /// for i in 0..81 {
    ///     // load page
    ///     thread::sleep(time::Duration::from_millis(50));
    /// 
    ///     // log the result
    ///     if i == 14 {
    ///         progress_bar.print_info("Failed", "https://zefzef.zef", Color::Red, Style::Blink);
    ///     } else {
    ///         progress_bar.print_info("Success", "https://example.com", Color::Red, Style::Blink);
    ///     }
    ///     
    ///     // update the progression by 1
    ///     progress_bar.inc();
    /// }
    /// progress_bar.print_final_info("Loading", "Load complete", Color::LightGreen, Style::Bold);
    /// // Or, to leave the progress bar at 100%:
    /// // progress_bar.finalize();
    /// ```
    pub fn new(max: usize) -> Self {
        ProgressBar {
            max,
            progression: 0,
            width: 50,
            action: String::new(),
            action_color: Color::Black,
            action_style: Style::Normal,
            start: None,
        }
    }

    /// Same as [ProgressBar::new] but enabled ETA display.
    pub fn new_with_eta(max: usize) -> Self {
        ProgressBar {
            start: Some(Instant::now()),
            ..ProgressBar::new(max)
        }
    }

    fn set_good_size(text: &str) -> String {
        match text.len() {
            12 => text.to_string(),
            len if len > 12 => text[..12].to_string(),
            _ => {
                let mut text = text.to_string();
                while text.len() < 12 {
                    text.insert(0, ' ');
                }
                text
            },
        }
    }

    /// Set the width of the progress bar in caracters in console (default: 50)
    pub fn set_width(&mut self, w: usize) {
        self.width = w;
        self.display();
    }

    /// Set the progression
    pub fn set_progression(&mut self, p: usize) {
        self.progression = p;
        if self.start.is_some() {
            self.start = Some(Instant::now());
        }
        self.display();
    }

    /// Set the max progression
    pub fn set_max(&mut self, m: usize) {
        self.max = m;
        self.display();
    }

    /// Increment the progression by 1
    pub fn inc(&mut self) {
        self.progression += 1;
        self.display();
    }

    /// **Resets progress** and enables ETA
    pub fn enable_eta(&mut self) {
        self.progression = 0;
        self.start = Some(Instant::now());
    }

    /// Disables ETA
    pub fn disable_eta(&mut self) {
        self.start = None;
    }

    /// Set the global action displayed before the progress bar.
    pub fn set_action(&mut self, a: &str, c: Color, s: Style) {
        self.action = ProgressBar::set_good_size(a);
        self.action_color = c;
        self.action_style = s;
        self.display();
    }

    /// Log something, without display update
    pub fn print_final_info(&mut self, info_name: &str, text: &str, info_color: Color, info_style: Style) {
        let info_name = ProgressBar::set_good_size(info_name);
        println!("{}{}{}\x1B[0m {}\x1B[K", info_style, info_color, info_name, text);
        self.progression = 0;
    }

    /// Log something
    pub fn print_info(&mut self, info_name: &str, text: &str, info_color: Color, info_style: Style) {
        let info_name = ProgressBar::set_good_size(info_name);
        println!("{}{}{}\x1B[0m {}\x1B[K", info_style, info_color, info_name, text);
        self.display();
    }

    /// Display the bar
    pub fn display(&self) {
        print!("{}{}{}\x1B[0m\x1B[K", self.action_style, self.action_color, self.action);

        print!(" [");
        for i in 0..self.width {
            if i*self.max/self.width < self.progression {
                if (i+1)*self.max/self.width >= self.progression {
                    print!(">");
                } else {
                    print!("=");
                }
            } else {
                print!(" ");
            }
        }
        print!("] {}/{}", self.progression, self.max);
        if let Some(start) = self.start {
            if self.max != 0 && self.progression != 0 && self.progression != self.max {
                let elapsed = start.elapsed();
                let progress_rate = self.progression as f64 / self.max as f64;
                let inv_progress_rate = 1. - progress_rate;
                let total_time = elapsed.as_millis() as f64 / progress_rate;
                let remaining_time = total_time * inv_progress_rate;
                let remaining_ms = remaining_time.ceil() as usize;

                const SECS_110: usize = 110 * 1000;
                const MINS_110: usize = 110 * 60 * 1000;
                const HOURS_46: usize = 46 * 60 * 60 * 1000;

                #[allow(overlapping_range_endpoints)]
                #[allow(clippy::match_overlapping_arm)]
                let eta = match remaining_ms {
                    0..=3_000 => format!("{}ms", remaining_time.ceil() as usize),
                    3_001..=SECS_110 => format!("{}s", (remaining_time / 1000.).ceil() as usize),
                    SECS_110..=MINS_110 => format!("{} minutes", (remaining_time / (1000. * 60.)).ceil() as usize),
                    MINS_110..=HOURS_46 => format!("{} hours", (remaining_time / (1000. * 60. * 60.)).ceil() as usize),
                    _ => format!("{} days", (remaining_time / (1000. * 60. * 60. * 24.)).ceil() as usize),
                };

                print!(" (ETA {eta})");
            }
        }
        print!("\n\x1B[1A");

        #[allow(unused_must_use)]
        { io::stdout().flush(); }
    }
    
    /// Mark the end of the progress bar - updates will make a 'new' bar
    pub fn finalize(&mut self) {
        self.progression = 0;
        println!();
    }
}
