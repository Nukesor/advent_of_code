use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;

/// Init better_panics and logging
pub fn init_app(verbosity: u8) {
    // Beautify panics for better debug output.
    better_panic::install();

    // Set the verbosity level and initialize the logger.
    let level = match verbosity {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        _ => LevelFilter::Debug,
    };
    Builder::new().filter_level(level).init();
}
