use crate::color::*;
use std::thread;
use std::time;

pub struct ProgressBar {
    max: usize,
    progression: usize,
    width: usize,
    action: String,
    action_color: Color,
    action_style: Style,
}

impl ProgressBar {
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

    pub fn set_width(&mut self, w: usize) {
        self.width = w;
        self.display();
    }

    pub fn set_progression(&mut self, p: usize) {
        self.progression = p;
        self.display();
    }

    pub fn inc(&mut self) {
        self.progression += 1;
        self.display();
    }

    pub fn set_action(&mut self, a: &str, c: Color, s: Style) {
        self.action = ProgressBar::set_good_size(a);
        self.action_color = c;
        self.action_style = s;
        self.display();
    }

    pub fn print_info(&mut self, info_name: &str, text: &str, info_color: Color, info_style: Style) {
        let info_name = ProgressBar::set_good_size(info_name);
        println!("{}{}{}\x1B[0m {}\x1B[K", info_style, info_color, info_name, text);
        self.display();
    }

    pub fn display(&self) {
        print!("{}{}{}\x1B[0m", self.action_style, self.action_color, self.action);

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
        print!("\n\x1B[1A")
    }

}