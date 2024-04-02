use crate::{
    hash::types::HashOutput,
    patricia_merkle_tree::filled_node::{ClassHash, Nonce},
    types::Felt,
};
use serde::Deserialize;
use std::collections::HashMap;

type DeserializedFelt = [u8; 32];
#[derive(PartialEq, Eq, Hash)]
pub(crate) struct StorageKey(pub Vec<u8>);
#[allow(dead_code)]
pub(crate) struct StorageValue(pub Vec<u8>);
#[derive(PartialEq, Eq, Hash)]
// TODO(Nimrod, 1/6/2024): Swap to starknet-types-core types once implemented.
pub(crate) struct ContractAddress(pub Felt);
#[derive(PartialEq, Eq, Hash)]
// TODO(Nimrod, 1/6/2024): Swap to starknet-types-core types once implemented.
pub(crate) struct StarknetStorageKey(pub Felt);

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
/// Input to the committer.
pub(crate) struct DeserializedInput {
    /// Storage. Will be casted to HashMap<Bytes, Bytes> to simulate DB access.
    pub storage: Vec<DeserializedStorageEntry>,
    /// All relevant information for the state diff commitment.
    pub state_diff: DeserializedStateDiff,
}

#[allow(dead_code)]
pub(crate) struct ActualInput {
    pub storage: HashMap<StorageKey, StorageValue>,
    pub state_diff: ActualStateDiff,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
/// Fact storage entry.
pub(crate) struct DeserializedStorageEntry {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct DeserializedFeltMapEntry {
    pub key: DeserializedFelt,
    pub value: DeserializedFelt,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
/// Represents storage updates. Later will be casted to HashMap<Felt, HashMap<Felt,Felt>> entry.
pub(crate) struct DeserializedStorageUpdates {
    pub address: DeserializedFelt,
    pub storage_updates: Vec<DeserializedFeltMapEntry>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
/// Represents current state leaf at the contract state tree. Later will be casted to
/// HashMap<Felt, (nonce, class_hash, storage_root_hash)> entry.
pub(crate) struct DeserializedContractStateLeaf {
    pub address: DeserializedFelt,
    pub nonce: DeserializedFelt,
    pub storage_root_hash: DeserializedFelt,
    pub class_hash: DeserializedFelt,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
/// Represents state diff.
pub(crate) struct DeserializedStateDiff {
    /// Will be casted to HashMap<Felt,Felt>.
    pub address_to_class_hash: Vec<DeserializedFeltMapEntry>,
    /// Will be casted to HashMap<Felt,Felt>.
    pub address_to_nonce: Vec<DeserializedFeltMapEntry>,
    /// Will be casted to HashMap<Felt,Felt>.
    pub class_hash_to_compiled_class_hash: Vec<DeserializedFeltMapEntry>,
    /// Will be casted to HashMap<Felt,HashMap<Felt,Felt>>.
    pub storage_updates: Vec<DeserializedStorageUpdates>,
    /// Will be casted to HashMap<Felt,ContractState>.
    pub current_contract_state_leaves: Vec<DeserializedContractStateLeaf>,
}

#[allow(dead_code)]
pub(crate) struct ActualStateDiff {
    pub address_to_class_hash: HashMap<ContractAddress, ClassHash>,
    pub address_to_nonce: HashMap<ContractAddress, Nonce>,
    pub class_hash_to_compiled_class_hash: HashMap<ClassHash, ClassHash>,
    pub current_contract_state_leaves: HashMap<ContractAddress, ContractState>,
    pub storage_updates: HashMap<ContractAddress, HashMap<StarknetStorageKey, Felt>>,
}

#[allow(dead_code)]
pub(crate) struct ContractState {
    pub nonce: Nonce,
    pub class_hash: ClassHash,
    pub storage_root_hash: HashOutput,
}
