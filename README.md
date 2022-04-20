# progress_bar

This crate allows you to display a progress bar in a terminal.

## Example

```rust
use progress_bar::progress_bar::ProgressBar;
use progress_bar::color::{Color, Style};
use std::{thread, time};

// if you have 81 pages to load
let mut progress_bar = ProgressBar::new(81);
progress_bar.set_action("Loading", Color::Blue, Style::Bold);

for i in 0..81 {
	// load page
	thread::sleep(time::Duration::from_millis(500));

	// log the result
	if i == 14 {
		progress_bar.print_info("Failed", "https://zefzef.zef", Color::Red, Style::Normal);
	} else if i == 41 {
		progress_bar.print_info("Success", "https://example.com", Color::Green, Style::Bold);
	}

	// update the progression by 1
	progress_bar.inc();
}
```

![image displaying the output of the code above](https://mubelotix.dev/images/progress_bar_example1.png "Output")

License: MIT
