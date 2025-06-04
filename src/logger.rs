use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use crate::{Color, Style, CURRENT_PROGRESS_BAR};

struct ProgressBarLogger {
    pub outer: Option<&'static dyn Log>,
    pub filter: &'static (dyn Fn(&Record) -> bool + Sync),
}

impl log::Log for ProgressBarLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if let Some(outer_logger) = &self.outer {
            outer_logger.enabled(metadata)
        } else {
            true // Default to enabled if no outer logger is set
        }
    }

    fn log(&self, record: &Record) {
        if !(*self.filter)(record) {
            return; // Skip logging if the filter does not match
        }

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
                if let Some(outer_logger) = &self.outer {
                    outer_logger.log(record);
                } else {
                    println!("{}: {}", record.level(), record.args());
                }
            }
        }
    }

    fn flush(&self) {}
}

/// Initializes the logger with options: 
/// - `fallback`: An optional logger that will be used when no progress bar is active. When `None`, calls to logging functions will print to stdout.
/// - `level`: The log level to set for the logger.
/// - `filter`: A function that takes a `&Record` and returns a boolean indicating whether the record should be logged.
/// 
/// ## Example: Integration with env_logger
/// 
/// Here is a snippet that shows how to integrate with `env_logger` so that:
/// - Logs are filtered according to the `env_logger` configuration
/// - When no progress bar is active, logs are printed using the `env_logger`
/// 
/// ```
/// use env_logger::Env;
/// # use progress_bar::init_logger_with_options;
/// # use log::*;
/// 
/// let fallback = env_logger::Builder::from_env(Env::default()).build();
/// let fallback = &*Box::leak(Box::new(fallback));
/// init_logger_with_options(Some(fallback), LevelFilter::Trace, |r| fallback.matches(r)).unwrap();
/// ```
pub fn init_logger_with_options(
    fallback: Option<impl Log + 'static>,
    level: LevelFilter,
    filter: impl Fn(&Record) -> bool + Sync + 'static,
) -> Result<(), SetLoggerError> {
    let fallback = Box::leak(Box::new(fallback));
    let logger = Box::leak(Box::new(ProgressBarLogger {
        outer: fallback.as_ref().map(|f| f as &dyn Log),
        filter: Box::leak(Box::new(filter)),
    }));
    log::set_logger(logger).map(|()| log::set_max_level(level))
}

/// Helper function to initialize the logger with default settings.
/// 
/// See [`init_logger_with_options`] for more details.
pub fn init_logger() -> Result<(), SetLoggerError> {
    init_logger_with_options(None::<&'static dyn Log>, LevelFilter::Trace, |_| true)
}

/// Helper function to initialize the logger just with a specific log level.
/// 
/// See [`init_logger_with_options`] for more options.
pub fn init_logger_with_level(level: LevelFilter) -> Result<(), SetLoggerError> {
    init_logger_with_options(None::<&'static dyn Log>, level, |_| true)
}

/// Helper function to initialize the logger just with a filter.
/// 
/// See [`init_logger_with_options`] for more options.
pub fn init_logger_with_filter(filter: fn(&Record) -> bool) -> Result<(), SetLoggerError> {
    init_logger_with_options(None::<&'static dyn Log>, LevelFilter::Trace, filter)
}

/// Helper function to initialize the logger just with a fallback logger.
/// 
/// See [`init_logger_with_options`] for more options.
pub fn init_logger_with_fallback(fallback: impl Log + 'static) -> Result<(), SetLoggerError> {
    init_logger_with_options(Some(fallback), LevelFilter::Trace, |_| true)
}

/// Helper function to initialize the logger with a fallback logger and a specific log level.
/// 
/// See [`init_logger_with_options`] for more options.
pub fn init_logger_with_fallback_and_level(
    fallback: impl Log + 'static,
    level: LevelFilter,
) -> Result<(), SetLoggerError> {
    init_logger_with_options(Some(fallback), level, |_| true)
}
