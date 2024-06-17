use crate::felt::Felt;
use crate::hash::hash_trait::HashOutput;
use crate::patricia_merkle_tree::filled_tree::node::CompiledClassHash;
use crate::patricia_merkle_tree::filled_tree::node::FilledNode;
use crate::patricia_merkle_tree::filled_tree::node::{ClassHash, Nonce};
use crate::patricia_merkle_tree::node_data::inner_node::NodeData;
use crate::patricia_merkle_tree::node_data::leaf::{ContractState, LeafDataImpl};
use crate::storage::db_object::{DBObject, Deserializable};
use crate::storage::storage_trait::StorageKey;

use rstest::rstest;

#[rstest]
#[case::zero_storage_leaf(LeafDataImpl::StorageValue(Felt::ZERO))]
#[case::non_zero_storage_leaf(LeafDataImpl::StorageValue(Felt::from(999_u128)))]
#[case::zero_compiled_class_leaf(LeafDataImpl::CompiledClassHash(CompiledClassHash(Felt::ZERO)))]
#[case::non_zero_compiled_class_leaf(LeafDataImpl::CompiledClassHash(CompiledClassHash(
    Felt::from(11_u128)
)))]
#[case::zero_contract_state_leaf(LeafDataImpl::ContractState(ContractState {
     nonce: Nonce(Felt::ZERO), storage_root_hash: HashOutput(Felt::ZERO), class_hash: ClassHash(Felt::ZERO)
    })
)]
#[case::partial_zero_contract_state_leaf(LeafDataImpl::ContractState(ContractState {
    nonce: Nonce(Felt::ZERO), storage_root_hash: HashOutput(Felt::from(2359743529034_u128)), class_hash: ClassHash(Felt::from(1349866415897798_u128))
   })
)]
#[case::without_zero_contract_state_leaf(LeafDataImpl::ContractState(ContractState {
    nonce: Nonce(Felt::from(23479515749555_u128)), storage_root_hash: HashOutput(Felt::from(2359743529034_u128)), class_hash: ClassHash(Felt::from(1349866415897798_u128))
   })
)]
fn test_leaf_serde(#[case] leaf: LeafDataImpl) {
    // For simplicity we use FilledNode as it already can serialize itself.
    let prefix = leaf.get_prefix();
    let node = FilledNode {
        hash: HashOutput::ROOT_OF_EMPTY_TREE,
        data: NodeData::Leaf(leaf),
    };
    let serialized_node_suffix = node.suffix();
    let serialized_node_value = node.serialize();
    let deserialized_node = FilledNode::deserialize(
        &StorageKey(serialized_node_suffix.to_vec()),
        &serialized_node_value,
        &prefix,
    )
    .unwrap();
    assert_eq!(node.data, deserialized_node.data);
}
