use progress_bar::progress_bar::ProgressBar;
use std::time;
use std::thread;

#[test]
fn test() {
    let mut test = ProgressBar::new(100);
    test.set_action("Liking");
    for _ in 0..100 {
        test.inc();
        thread::sleep(time::Duration::from_millis(50));
    }
    
}