use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use crate::{Color, Style, CURRENT_PROGRESS_BAR};

struct ProgressBarLogger(Option<&'static dyn Log>);

impl log::Log for ProgressBarLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if let Some(outer_logger) = self.0 {
            outer_logger.enabled(metadata)
        } else {
            true // Default to enabled if no outer logger is set
        }
    }

    fn log(&self, record: &Record) {
        match CURRENT_PROGRESS_BAR.lock().as_deref_mut() {
            Ok(Some(progress_bar)) => match record.level() {
                Level::Error => progress_bar.print_info("Error", &record.args().to_string(), Color::Red, Style::Bold),
                Level::Warn => progress_bar.print_info("Warn", &record.args().to_string(), Color::Yellow, Style::Bold),
                Level::Info => progress_bar.print_info("Info", &record.args().to_string(), Color::LightGreen, Style::Bold),
                Level::Debug => progress_bar.print_info("Debug", &record.args().to_string(), Color::Blue, Style::Normal),
                Level::Trace => progress_bar.print_info("Trace", &record.args().to_string(), Color::LightGray, Style::Normal),
            },
            Ok(None) | Err(_) => {
                // If no progress bar is set, we use the outer logger if available
                if let Some(outer_logger) = self.0 {
                    outer_logger.log(record);
                } else {
                    println!("{}: {}", record.level(), record.args());
                }
            }
        }
    }

    fn flush(&self) {}
}

/// Initializes the logger with a progress bar.
/// This logger initialization cannot be undone, and must only be called once.
/// Calls to `log::info!`, `log::warn!`, etc. will print messages to the progress bar if it is set.
/// The default log level is `Trace`, but you can change it using [`init_logger_with_level`] or [`log::set_max_level`].
/// 
/// When there is no progress bar set, it will just print raw messages directly to stdout.
/// If you want to use another logger as a fallback, use [`init_logger_with_fallback`].
pub fn init_logger() -> Result<(), SetLoggerError> {
    init_logger_with_level(LevelFilter::Trace)
}

/// Same as [`init_logger`] but allows you to set the log level.
pub fn init_logger_with_level(level: LevelFilter) -> Result<(), SetLoggerError> {
    let logger: &'static dyn Log = Box::leak(Box::new(ProgressBarLogger(None)));
    log::set_logger(logger).map(|()| log::set_max_level(level))
}

/// Initializes the logger with a progress bar and a fallback logger.
/// This logger initialization cannot be undone, and must only be called once.
/// Calls to `log::info!`, `log::warn!`, etc. will print messages to the progress bar if it is set.
/// The default log level is `Trace`, but you can change it using [`init_logger_with_fallback_and_level`] or [`log::set_max_level`].
/// 
/// When there is no progress bar set, it will use the provided fallback logger.
/// 
/// # Example using `env_logger` as a fallback:
/// ```
/// use env_logger::Env;
/// use progress_bar::init_logger_with_fallback;
/// 
/// let fallback = env_logger::Builder::from_env(Env::default()).build();
/// init_logger_with_fallback(fallback).expect("Failed to initialize logger with fallback");
/// ```
pub fn init_logger_with_fallback(fallback: impl Log + 'static) -> Result<(), SetLoggerError> {
    init_logger_with_fallback_and_level(fallback, LevelFilter::Trace)
}

/// Same as [`init_logger_with_fallback`] but allows you to set the log level.
pub fn init_logger_with_fallback_and_level(
    fallback: impl Log + 'static,
    level: LevelFilter,
) -> Result<(), SetLoggerError> {
    let fallback = Box::leak(Box::new(fallback));
    let logger = Box::leak(Box::new(ProgressBarLogger(Some(fallback))));
    log::set_logger(logger).map(|()| log::set_max_level(level))
}
