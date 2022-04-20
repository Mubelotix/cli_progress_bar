//! This crate allows you to display a pretty progress bar in the terminal.
//! 
//! **LINUX ONLY**
//! 
//! # Example
//! 
//! ```
//! use progress_bar::*
//! # use std::{thread::sleep, time::Duration};
//! 
//! // if you have 81 pages to load
//! init_progress_bar(81);
//! set_progress_bar_action("Loading", Color::Blue, Style::Bold);
//!
//! for i in 0..81 {
//!     // load page
//!     sleep(Duration::from_millis(500));
//!
//!     // log the result
//!     if i == 14 {
//!         print_progress_bar_info("Failed", "https://zefzef.zef", Color::Red, Style::Normal);
//!     } else if i == 41 {
//!         print_progress_bar_info("Success", "https://example.com", Color::Green, Style::Bold);
//!     }
//!     
//!     // update the progression by 1
//!     inc_progress_bar();
//! }
//! 
//! finalize_progress_bar();
//! ```
//! 
//! ![image displaying the output of the code above](https://mubelotix.dev/images/progress_bar_example1.png "Output")

/// Includes the [`ProgressBar`] struct.
pub mod pb;
/// Includes the [`Color`] and [`Style`] enums.
pub mod style;
/// Includes functions to use a global progress bar (recommended).
pub mod singleton;

pub use style::*;
pub use singleton::*;
