//! The main module

use crate::color::*;
use std::io;
use std::io::Write;

pub struct ProgressBar {
    max: usize,
    progression: usize,
    width: usize,
    action: String,
    action_color: Color,
    action_style: Style,
}

impl ProgressBar {
    /// Creates a progress bar with the total number of actions.  
    /// You will need to call inc method when an action is completed and the bar progression will be incremented by 1.  
    /// Don't print things with print macro while the bar is running; use the print_info method instead.  
    /// 
    /// Example:
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
            action_style: Style::Normal
        }
    }

    fn set_good_size(text: &str) -> String {
        if text.len() == 12 {
            text.to_string()
        } else if text.len() > 12 {
            text[..12].to_string()
        } else {
            let mut text = text.to_string();
            while text.len() < 12 {
                text.insert(0, ' ');
            }
            text
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
        self.display();
    }

    /// Increment the progression by 1
    pub fn inc(&mut self) {
        self.progression += 1;
        self.display();
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
        print!("\n\x1B[1A");

        #[allow(unused_must_use)]
        { io::stdout().flush(); }
    }
    
    /// Mark the end of the progress bar - updates will make a 'new' bar
    pub fn finalize(&mut self) {
        self.progression = 0;
        println!("");
    }
}
