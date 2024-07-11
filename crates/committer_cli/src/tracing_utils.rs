use tracing::metadata::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

const DEFAULT_LEVEL: LevelFilter = LevelFilter::INFO;

// TODO(Amos, 1/8/2024): Actually understand the code in this file and assert it's what we need.
// It is the same as Papyrus and mempool-dev, except I added `.with_ansi(false)`
// TODO(Amos, 1/8/2024): Delete the below TODOs or move to my name.
// TODO(yair): add dynamic level filtering.
// TODO(dan): filter out logs from dependencies (happens when RUST_LOG=DEBUG)
// TODO(yair): define and implement configurable filtering.
pub fn configure_tracing() {
    let fmt_layer = fmt::layer().compact().with_target(false).with_ansi(false);
    let level_filter_layer = EnvFilter::builder()
        .with_default_directive(DEFAULT_LEVEL.into())
        .from_env_lossy();

    // This sets a single subscriber to all of the threads. We may want to implement different
    // subscriber for some threads and use set_global_default instead of init.
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(level_filter_layer)
        .init();
}
