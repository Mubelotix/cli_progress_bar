//! This crate allows you to display progress bar in a terminal.
//! 
//! # Example
//! 
//! ```
//! use progress_bar::progress_bar::ProgressBar;
//! use progress_bar::color::{Color, Style};
//! use std::{thread, time};
//! 
//! // if you have 81 pages to load
//! let mut progress_bar = ProgressBar::new(81);
//! progress_bar.set_action("Loading", Color::Blue, Style::Bold);
//!
//! for i in 0..81 {
//!     // load page
//!     thread::sleep(time::Duration::from_millis(50));
//! 
//!     // log the result
//!     if i == 14 {
//!         progress_bar.print_info("Failed", "https://zefzef.zef", Color::Red, Style::Blink);
//!     } else {
//!         progress_bar.print_info("Success", "https://example.com", Color::Red, Style::Blink);
//!     }
//!     
//!     // update the progression by 1
//!     progress_bar.inc();
//! }
//! ```

pub mod progress_bar;
pub mod color;

