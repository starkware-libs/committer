use derive_more::Display;

#[derive(Debug)]
pub(crate) enum StorageError {}

#[derive(thiserror::Error, Debug, Display)]
#[allow(dead_code)]
pub(crate) enum SerdeError {
    DeserializeError,
    SerializeError,
}
