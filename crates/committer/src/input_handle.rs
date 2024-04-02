use crate::hash::types::HashOutput;
use crate::patricia_merkle_tree::filled_node::{ClassHash, LeafData, Nonce};
use crate::patricia_merkle_tree::skeleton_node::SkeletonNode;
use crate::patricia_merkle_tree::types::{EdgePath, EdgePathLength, NodeIndex, PathToBottom};
use serde::Deserialize;
use starknet_types_core::felt::Felt;
use std::collections::HashMap;
#[allow(dead_code)]

pub(super) fn parse_input(
    input: String,
) -> (
    HashMap<NodeIndex, SkeletonNode<LeafData>>,
    HashMap<NodeIndex, SkeletonNode<LeafData>>,
) {
    let deserialized_input: DeserializedInput =
        serde_json::from_str(&input).expect("Error at parsing python input.");
    let mut leaves = HashMap::default();
    for leaf in deserialized_input.leaves {
        let (idx, leaf_data) = leaf.to_indexed_leaf();
        // Make sure there are no duplicates.
        assert!(leaves.insert(idx, SkeletonNode::Leaf(leaf_data)).is_none());
    }
    let mut prefetched_nodes = HashMap::default();
    for prefetched_node in deserialized_input.prefetched_nodes {
        let (idx, node) = prefetched_node.to_indexed_node();
        // Make sure there are no duplicates.
        assert!(!leaves.contains_key(&idx) && prefetched_nodes.insert(idx, node).is_none())
    }

    (leaves, prefetched_nodes)
}
#[derive(Debug, Deserialize)]
#[allow(dead_code)]

pub(crate) struct DeserializedInput {
    prefetched_nodes: Vec<DeserializedIndexedNode>,
    leaves: Vec<DeserializedIndexedLeaf>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]

pub(crate) struct DeserailizedLeaf {
    leaf_type: u8,
    contract_hash: Vec<u8>,
    stroage_root: Vec<u8>,
    value: Vec<u8>,
}
#[derive(Deserialize, Debug)]
#[allow(dead_code)]

pub(crate) struct DeserializedIndexedLeaf {
    index: Vec<u8>,
    leaf: DeserailizedLeaf,
}
#[allow(dead_code)]

impl DeserializedIndexedLeaf {
    pub(crate) fn to_indexed_leaf(&self) -> (NodeIndex, LeafData) {
        // TODO(Nimrod, 10/4/2024): This function should accept LeafPrefixes once CLI interface
        // is merged.
        let storage_leaf_type = 1;
        let contract_state_leaf_type = 2;
        let contract_class_leaf_type = 3;
        let index = NodeIndex(Felt::from_bytes_be_slice(&self.index));
        if self.leaf.leaf_type == storage_leaf_type {
            return (
                index,
                LeafData::StorageValue(Felt::from_bytes_be_slice(&self.leaf.value)),
            );
        }
        if self.leaf.leaf_type == contract_state_leaf_type {
            (
                index,
                LeafData::StateTreeTuple {
                    class_hash: ClassHash(Felt::from_bytes_be_slice(&self.leaf.contract_hash)),
                    contract_state_root_hash: Felt::from_bytes_be_slice(&self.leaf.stroage_root),
                    nonce: Nonce(Felt::from_bytes_be_slice(&self.leaf.value)),
                },
            )
        } else {
            assert_eq!(self.leaf.leaf_type, contract_class_leaf_type);
            (
                index,
                LeafData::CompiledClassHash(ClassHash(Felt::from_bytes_be_slice(&self.leaf.value))),
            )
        }
    }
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct DeserializedNode {
    node_type: u8,
    length: u16,
    path: Vec<u8>,
    value: Vec<u8>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct DeserializedIndexedNode {
    index: Vec<u8>,
    node: DeserializedNode,
}
#[allow(dead_code)]
#[allow(unreachable_code)]
impl DeserializedIndexedNode {
    pub(crate) fn to_indexed_node(&self) -> (NodeIndex, SkeletonNode<LeafData>) {
        // TODO(Nimrod, 10/4/2024): This function should accept NodePrefixes once CLI interface
        // is merged.
        let sibling_node_type = 1;
        let empty_node_type = 2;
        let edge_node_type = 3;
        let binary_node_type = 4;
        let index = NodeIndex(Felt::from_bytes_be_slice(&self.index));
        if self.node.node_type == sibling_node_type {
            return (
                index,
                SkeletonNode::Sibling(HashOutput(Felt::from_bytes_be_slice(&self.node.value))),
            );
        }
        if self.node.node_type == edge_node_type {
            let length = EdgePathLength(self.node.length);
            let path = EdgePath(Felt::from_bytes_be_slice(&self.node.path));
            return (
                index,
                SkeletonNode::Edge {
                    path_to_bottom: PathToBottom { length, path },
                },
            );
        }
        if self.node.node_type == empty_node_type {
            (index, SkeletonNode::Empty)
        } else {
            assert_eq!(self.node.node_type, binary_node_type);
            (index, SkeletonNode::Binary)
        }
    }
}
