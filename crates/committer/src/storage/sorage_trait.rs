pub(crate) trait Storage{
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>);
    fn remove(&mut self, key: &[u8]);
    fn new() -> Self;
}