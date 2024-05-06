use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum TypesError {
    #[error("Failed to convert type {from} to {to}. Reason: {reason}.")]
    ConversionError {
        from: &'static str,
        to: &'static str,
        reason: &'static str,
    },
}
