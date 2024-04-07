
use std::collections::HashMap;
use super::sorage_trait::Storage;
use crate::patricia_merkle_tree::filled_node::{FilledNode, LeafData};


pub(crate) struct CommitterStorage {
    storage: HashMap<Vec<u8>, Vec<u8>>,
    node_prefix: &'static [u8],
}

impl CommitterStorage {
    pub(crate) fn set_filled_node(&mut self, node: &FilledNode<LeafData>){
        let key = self.node_prefix.extend_from_slice();
        let mut value = Vec::new();
        value.extend_from_slice(&node.node_index.0.to_be_bytes());
        value.extend_from_slice(&node.node_data.hash);
        self.storage.insert(key, value);
    }
}

impl Storage for CommitterStorage {

    fn new() -> Self {
        Self {
            storage: HashMap::new(),
            node_prefix: b"patricia_node",
        }
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.storage.get(key).cloned()
    }

    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.storage.insert(key, value);
    }

    fn remove(&mut self, key: &[u8]) {
        self.storage.remove(key);
    }



}