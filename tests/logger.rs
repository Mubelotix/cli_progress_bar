#![cfg(feature = "logger")]

#[test]
fn test() {
    use progress_bar::*;
    use log::*;
    use std::thread::sleep;
    use std::time::Duration;
    use env_logger::Env;

    let fallback = env_logger::Builder::from_env(Env::default()).build();
    let fallback = &*Box::leak(Box::new(fallback));

    init_logger_with_options(Some(fallback), LevelFilter::Trace, |r| fallback.matches(r)).unwrap();

    info!("You can print even when no progress bar is active");

    init_progress_bar(100);

    info!("Loading website https://example.com");
    warn!("Failed to load https://zefzef.zef");

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
