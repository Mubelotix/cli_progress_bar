#![cfg(feature = "logger")]

#[test]
fn test() {
    use progress_bar::*;
    use log::*;
    use std::thread::sleep;
    use std::time::Duration;

    init_logger().unwrap();
    info!("You can log before the progress bar is initialized");

    // if you have 81 pages to load
    init_progress_bar_with_eta(81);
    set_progress_bar_action("Loading", Color::Blue, Style::Bold);
    
    for i in 0..81 {
        // load page
        sleep(Duration::from_millis(100));
    
        // log the result
        if i == 14 {
            error!("Failed to load https://zefzef.zef");
        } else if i == 41 {
            info!("Success loading https://example.com");
        }
    
        // increase the progress by 1
        inc_progress_bar();
    }
    
    finalize_progress_bar();
}
