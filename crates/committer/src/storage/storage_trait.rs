pub(crate) trait Storage {
    /// Returns value from storage, if it exists.
    fn get(&self, key: &[u8]) -> Option<&[u8]>;
    /// Sets value in storage.
    fn set(&mut self, key: &[u8], value: &[u8]);
    /// Returns values from storage in same order of given keys. If key does not exist,
    /// value is None.
    fn mget(&self, keys: Vec<&[u8]>) -> Vec<Option<&[u8]>>;
    /// Sets values in storage.
    fn mset(&mut self, key: Vec<&[u8]>, value: Vec<&[u8]>);
    /// Deletes value from storage.
    fn delete(&mut self, key: &[u8]);
}
