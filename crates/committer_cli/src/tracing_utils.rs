use tracing::metadata::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

const DEFAULT_LEVEL: LevelFilter = LevelFilter::INFO;

// // TODO(dan): filter out logs from dependencies (happens when RUST_LOG=DEBUG)
// pub fn configure_tracing() {
//     let fmt_layer = fmt::layer()
//         .compact()
//         .with_span_events(FmtSpan::CLOSE | FmtSpan::ENTER)
//         .with_ansi(false);
//     let level_filter_layer = EnvFilter::builder()
//         .with_default_directive(DEFAULT_LEVEL.into())
//         .from_env_lossy();

//     // This sets a single subscriber to all of the threads.
//     tracing_subscriber::registry()
//         .with(fmt_layer)
//         .with(level_filter_layer)
//         .init();
// }

pub fn configure_tracing() {
    tracing_subscriber::fmt().init();
}
