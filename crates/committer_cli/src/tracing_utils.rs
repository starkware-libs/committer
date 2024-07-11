use tracing_subscriber::fmt;

pub fn configure_tracing() {
    fmt()
        .with_ansi(false)
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .init();
}
