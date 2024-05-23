use std::collections::HashMap;

use crate::felt::Felt;
use crate::hash::hash_trait::HashOutput;
use crate::patricia_merkle_tree::filled_tree::forest::FilledForestImpl;
use crate::patricia_merkle_tree::filled_tree::node::ClassHash;
use crate::patricia_merkle_tree::filled_tree::node::FilledNode;
use crate::patricia_merkle_tree::filled_tree::node::Nonce;
use crate::patricia_merkle_tree::filled_tree::tree::FilledTreeImpl;
use crate::patricia_merkle_tree::node_data::inner_node::BinaryData;
use crate::patricia_merkle_tree::node_data::inner_node::EdgeData;
use crate::patricia_merkle_tree::node_data::inner_node::{
    EdgePath, EdgePathLength, NodeData, PathToBottom,
};
use crate::patricia_merkle_tree::node_data::leaf::ContractState;
use crate::patricia_merkle_tree::node_data::leaf::LeafDataImpl;
use crate::patricia_merkle_tree::types::NodeIndex;
use ethnum::U256;
use rand::Rng;

/// Generates a random U256 number between low and high (exclusive).
/// Panics if low > high.
pub(crate) fn get_random_u256(low: U256, high: U256) -> U256 {
    assert!(low < high);
    let high_of_low = low.high();
    let high_of_high = high.high();

    let delta = high - low;
    if delta <= u128::MAX {
        let delta = u128::try_from(delta).expect("Failed to create a random U256");
        return low + rand::thread_rng().gen_range(0..delta);
    }

    // Randomize the high 128 bits in the extracted range, and the low 128 bits in their entire
    // domain until the result is in range.
    // As high-low>u128::MAX, the expected number of samples until the loops breaks is bound from
    // above by 3 (as either:
    //  1. high_of_high > high_of_low + 1, and there is a 1/3 chance to get a valid result for high
    //  bits in (high_of_low, high_of_high).
    //  2. high_of_high == high_of_low + 1, and every possible low 128 bits value is valid either
    // when the high bits equal high_of_high, or when they equal high_of_low).
    let randomize = || {
        U256::from_words(
            rand::thread_rng().gen_range(*high_of_low..=*high_of_high),
            rand::thread_rng().gen_range(0..=u128::MAX),
        )
    };
    let mut result = randomize();
    while result < low || result >= high {
        result = randomize();
    }
    result
}

pub trait RandomValue {
    fn random() -> Self;
}

pub trait DumpyRandomValue {
    fn dumpy_random() -> Self;
}

impl RandomValue for Felt {
    fn random() -> Self {
        Felt::from(rand::random::<u128>())
    }
}

impl RandomValue for HashOutput {
    fn random() -> Self {
        HashOutput(Felt::random())
    }
}

impl RandomValue for LeafDataImpl {
    fn random() -> Self {
        match rand::thread_rng().gen_range(0..3) {
            0 => LeafDataImpl::StorageValue(Felt::random()),
            1 => LeafDataImpl::CompiledClassHash(ClassHash(Felt::random())),
            2 => LeafDataImpl::ContractState(ContractState {
                nonce: Nonce(Felt::random()),
                storage_root_hash: HashOutput::random(),
                class_hash: ClassHash(Felt::random()),
            }),
            _ => unreachable!(),
        }
    }
}

impl RandomValue for BinaryData {
    fn random() -> Self {
        Self {
            left_hash: HashOutput::random(),
            right_hash: HashOutput::random(),
        }
    }
}

impl RandomValue for PathToBottom {
    fn random() -> Self {
        Self {
            path: EdgePath::random(),
            length: EdgePathLength::random(),
        }
    }
}

impl RandomValue for EdgePath {
    fn random() -> Self {
        Self(get_random_u256(U256::ONE, U256::ONE << 128))
    }
}

impl RandomValue for EdgePathLength {
    fn random() -> Self {
        Self(rand::random::<u8>())
    }
}

impl RandomValue for EdgeData {
    fn random() -> Self {
        Self {
            bottom_hash: HashOutput::random(),
            path_to_bottom: PathToBottom::random(),
        }
    }
}

impl RandomValue for NodeData<LeafDataImpl> {
    fn random() -> Self {
        match rand::thread_rng().gen_range(0..3) {
            0 => NodeData::Binary(BinaryData::random()),
            1 => NodeData::Edge(EdgeData::random()),
            2 => NodeData::Leaf(LeafDataImpl::random()),
            _ => unreachable!(),
        }
    }
}

impl RandomValue for NodeIndex {
    fn random() -> Self {
        Self::new(get_random_u256(U256::ONE, U256::ONE << 128))
    }
}

impl RandomValue for FilledNode<LeafDataImpl> {
    fn random() -> Self {
        Self {
            data: NodeData::random(),
            hash: HashOutput::random(),
        }
    }
}

impl DumpyRandomValue for FilledTreeImpl {
    /// Generates a dumpy random filled tree.
    /// The tree contains up to 101 random nodes in random indexes.
    /// Do not necessary represent a valid tree.
    fn dumpy_random() -> Self {
        let mut tree_map: HashMap<NodeIndex, FilledNode<LeafDataImpl>> = HashMap::new();
        // Insert the root node.
        tree_map.insert(NodeIndex::ROOT, FilledNode::random());
        // Insert the rest of the nodes.
        for _i in 0..100_u128 {
            let node_index = NodeIndex::random();
            let node = FilledNode::random();
            tree_map.insert(node_index, node);
        }

        Self { tree_map }
    }
}

impl DumpyRandomValue for FilledForestImpl {
    /// Generates a dumpy random filled forest.
    /// The forest contains 100 dumpy random storage trees,
    /// a dumpy random contract tree and a dumpy random compiled class tree.
    /// Does not necessary represent a valid forest.
    fn dumpy_random() -> Self {
        let mut storage_trees: HashMap<NodeIndex, FilledTreeImpl> = HashMap::new();
        for _i in 0..100_u128 {
            let node_index = NodeIndex::random();
            let tree = FilledTreeImpl::dumpy_random();
            storage_trees.insert(node_index, tree);
        }

        let contract_tree = FilledTreeImpl::dumpy_random();
        let compiled_class_tree = FilledTreeImpl::dumpy_random();

        Self {
            storage_trees,
            contract_tree,
            compiled_class_tree,
        }
    }
}
