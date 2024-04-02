use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub(crate) enum DeserializationError {
    #[error("There is a key duplication at {0} mapping.")]
    KeyDuplicate(String),
}
