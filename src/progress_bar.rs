use crate::color::*;
use std::thread;
use std::time;

pub struct ProgressBar {
    max: usize,
    progression: usize,
    width: usize,
    action: String,
}

impl ProgressBar {
    pub fn new(max: usize) -> Self {
        ProgressBar {
            max,
            progression: 0,
            width: 50,
            action: String::new(),
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

    pub fn set_action(&mut self, a: &str) {
        self.action = String::from(a);
        while self.action.len() > 12 {
            self.action.remove(12);
        }
        while self.action.len() < 12 {
            self.action.insert(0, ' ');
        }
        self.display();
    }

    pub fn display(&self) {
        print!("{}", self.action);

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