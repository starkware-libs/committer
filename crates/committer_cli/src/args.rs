#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct StoragePrefix(pub u8);

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct LeafPrefixes {
    pub(crate) storage: StoragePrefix,
    pub(crate) contract_state: StoragePrefix,
    pub(crate) class: StoragePrefix,
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct NodePrefixes {
    pub(crate) edge: StoragePrefix,
    pub(crate) sibling: StoragePrefix,
    pub(crate) binary: StoragePrefix,
    pub(crate) empty: StoragePrefix,
}

#[allow(dead_code)]
#[derive(Debug)]
/// Holds all the information needed for the committer.
pub(crate) enum InputArgs {
    CurrentArgs {
        input_path: String,
        output_path: String,
        class_hash_version: u8,
        contract_state_hash_version: u8,
        leaf_prefixes: LeafPrefixes,
        node_prefixes: NodePrefixes,
    },
}
