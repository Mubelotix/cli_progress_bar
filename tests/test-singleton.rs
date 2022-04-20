use progress_bar::*;

#[test]
fn test() {
    init_progress_bar(100);
    set_progress_bar_action("Loading", Color::Blue, Style::Bold);
    
    for i in 0..100 {
        inc_progress_bar();
        if i == 14 {
            print_progress_bar_info("Error", "loading something", Color::Red, Style::Blink)
        } else if i == 48 {
            print_progress_bar_info("Found", "something", Color::Green, Style::Bold)
        } else if i == 75 {
            print_progress_bar_info("Warning", "potential error", Color::Yellow, Style::Normal)
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    finalize_progress_bar();
}
