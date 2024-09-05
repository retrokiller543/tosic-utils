#![cfg(feature = "log")]
use env_logger::{Builder, Env};
use std::io::Write;
use tracing::Subscriber;
use tracing_subscriber::fmt::format::FmtSpan;

#[cfg(feature = "log")]
macro_rules! get_logger {
    ($default_level:expr) => {
        Builder::from_env(Env::default().default_filter_or($default_level)).format(|buf, record| {
            let ts = buf.timestamp();
            let line = record.line().unwrap_or(0);

            writeln!(
                buf,
                "{} [{}] [{}:{}] [{}] - {}",
                ts,
                record.level(),
                record.target(),
                line,
                std::thread::current().name().unwrap_or("unknown-thread"),
                record.args()
            )
        })
    };
}

#[cfg(feature = "log")]
#[inline]
pub fn init_logger(default_level: &str) {
    get_logger!(default_level).init()
}

#[cfg(feature = "log")]
#[inline]
pub fn init_test_logger(default_level: &str) {
    get_logger!(default_level).is_test(true).init()
}

#[cfg(feature = "tracing")]
pub fn default_tracing_subscriber() -> impl Subscriber {
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_file(true)
        .with_span_events(FmtSpan::FULL)
        .with_level(true)
        .with_target(true)
        .with_thread_names(true)
        .finish()
}

#[cfg(feature = "tracing")]
#[inline]
pub fn init_tracing() -> Result<(), String> {
    tracing::subscriber::set_global_default(default_tracing_subscriber())
        .map_err(|err| err.to_string())?;

    Ok(())
}
