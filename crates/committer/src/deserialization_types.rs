use crate::patricia_merkle_tree::filled_node::{ClassHash, Nonce};
use serde::Deserialize;
use starknet_types_core::felt::Felt;
use std::collections::HashMap;

type DeserializedFelt = Vec<u8>;
type Bytes = Vec<u8>;
type Address = Felt;

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
    pub storage: HashMap<Bytes, Bytes>,
    pub state_diff: ActualStateDiff,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
/// Fact storage entry.
pub(crate) struct DeserializedStorageEntry {
    pub key: Bytes,
    pub value: Bytes,
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
    pub address_to_class_hash: HashMap<Address, ClassHash>,
    pub address_to_nonce: HashMap<Address, Nonce>,
    pub class_hash_to_compiled_class_hash: HashMap<ClassHash, ClassHash>,
    pub current_contract_state_leaves: HashMap<Address, ContractState>,
    pub storage_updates: HashMap<Address, HashMap<Address, Felt>>,
}

#[allow(dead_code)]
pub(crate) struct ContractState {
    pub nonce: Nonce,
    pub class_hash: ClassHash,
    pub storage_root_hash: Felt,
}
