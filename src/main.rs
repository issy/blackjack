use std::panic;
use tracing::Level;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod game;
mod tui;

fn install_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        tracing::error!("panic: {panic_info}");

        let bt = std::backtrace::Backtrace::force_capture();
        tracing::error!("{bt}");
    }));
}

fn init_logging() {
    let file_appender = tracing_appender::rolling::never(".", "app.log")
        .with_min_level(Level::TRACE)
        .with_max_level(Level::ERROR);
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(file_appender).with_ansi(false))
        .init();
}

fn main() {
    init_logging();
    install_panic_hook();
    tracing::info!("Logging initialised");
    tracing::info!("Starting application");
    if let Err(err) = tui::entrypoint::start_app() {
        tracing::error!("Error occurred: {}", err);
    }
}
