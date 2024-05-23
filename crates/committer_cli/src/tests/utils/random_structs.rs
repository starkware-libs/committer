use std::collections::HashMap;

use committer::felt::Felt;
use committer::hash::hash_trait::HashOutput;
use committer::patricia_merkle_tree::filled_tree::forest::FilledForestImpl;
use committer::patricia_merkle_tree::filled_tree::node::ClassHash;
use committer::patricia_merkle_tree::filled_tree::node::FilledNode;
use committer::patricia_merkle_tree::filled_tree::node::Nonce;
use committer::patricia_merkle_tree::filled_tree::tree::FilledTreeImpl;
use committer::patricia_merkle_tree::node_data::inner_node::BinaryData;
use committer::patricia_merkle_tree::node_data::inner_node::EdgeData;
use committer::patricia_merkle_tree::node_data::inner_node::NodeDataDiscriminants as NodeDataVariants;
use committer::patricia_merkle_tree::node_data::inner_node::{
    EdgePath, EdgePathLength, NodeData, PathToBottom,
};
use committer::patricia_merkle_tree::node_data::leaf::ContractState;
use committer::patricia_merkle_tree::node_data::leaf::LeafDataImpl;
use committer::patricia_merkle_tree::node_data::leaf::LeafDataImplDiscriminants as LeafDataVariants;
use committer::patricia_merkle_tree::test_utils::get_random_u256;
use committer::patricia_merkle_tree::types::NodeIndex;
use ethnum::U256;
use rand::prelude::IteratorRandom;
use rand::Rng;
use strum::IntoEnumIterator;

pub trait RandomValue {
    fn random<R: Rng>(rng: &mut R, max: Option<U256>) -> Self;
}

pub trait DumpyRandomValue {
    fn dumpy_random<R: Rng>(rng: &mut R, max: Option<U256>) -> Self;
}

impl RandomValue for Felt {
    fn random<R: Rng>(_rng: &mut R, _max: Option<U256>) -> Self {
        Felt::from(&get_random_u256(U256::ONE, U256::from(&Felt::MAX) + 1))
    }
}

impl RandomValue for HashOutput {
    fn random<R: Rng>(rng: &mut R, max: Option<U256>) -> Self {
        HashOutput(Felt::random(rng, max))
    }
}

impl RandomValue for LeafDataImpl {
    fn random<R: Rng>(rng: &mut R, max: Option<U256>) -> Self {
        match LeafDataVariants::iter()
            .choose(rng)
            .expect("Failed to choose a random variant for LeafDataImpl")
        {
            LeafDataVariants::StorageValue => LeafDataImpl::StorageValue(Felt::random(rng, max)),
            LeafDataVariants::CompiledClassHash => {
                LeafDataImpl::CompiledClassHash(ClassHash(Felt::random(rng, max)))
            }
            LeafDataVariants::ContractState => LeafDataImpl::ContractState(ContractState {
                nonce: Nonce(Felt::random(rng, max)),
                storage_root_hash: HashOutput::random(rng, max),
                class_hash: ClassHash(Felt::random(rng, max)),
            }),
        }
    }
}

impl RandomValue for BinaryData {
    fn random<R: Rng>(rng: &mut R, max: Option<U256>) -> Self {
        Self {
            left_hash: HashOutput::random(rng, max),
            right_hash: HashOutput::random(rng, max),
        }
    }
}

impl RandomValue for PathToBottom {
    fn random<R: Rng>(rng: &mut R, max: Option<U256>) -> Self {
        Self {
            path: EdgePath::random(rng, max),
            length: EdgePathLength::random(rng, max),
        }
    }
}

impl RandomValue for EdgePath {
    fn random<R: Rng>(_rng: &mut R, max: Option<U256>) -> Self {
        // The maximum value is the maximum between max and EdgePath::MAX.
        let max_value = match max {
            Some(m) if m < EdgePath::MAX.0 => m,
            _ => EdgePath::MAX.0,
        };

        Self(get_random_u256(U256::ONE, max_value + 1))
    }
}

impl RandomValue for EdgePathLength {
    fn random<R: Rng>(rng: &mut R, max: Option<U256>) -> Self {
        // The maximum value is the maximum between max and EdgePathLength::MAX.
        let max_value = match max {
            Some(m) if m.as_u8() < EdgePathLength::MAX.0 => m.as_u8(),
            _ => EdgePathLength::MAX.0,
        };
        Self(rng.gen_range(1..max_value))
    }
}

impl RandomValue for EdgeData {
    fn random<R: Rng>(rng: &mut R, max: Option<U256>) -> Self {
        Self {
            bottom_hash: HashOutput::random(rng, max),
            path_to_bottom: PathToBottom::random(rng, max),
        }
    }
}

impl RandomValue for NodeData<LeafDataImpl> {
    fn random<R: Rng>(rng: &mut R, max: Option<U256>) -> Self {
        match NodeDataVariants::iter()
            .choose(rng)
            .expect("Failed to choose a random variant for NodeData")
        {
            NodeDataVariants::Binary => NodeData::Binary(BinaryData::random(rng, max)),
            NodeDataVariants::Edge => NodeData::Edge(EdgeData::random(rng, max)),
            NodeDataVariants::Leaf => NodeData::Leaf(LeafDataImpl::random(rng, max)),
        }
    }
}

impl RandomValue for NodeIndex {
    fn random<R: Rng>(_rng: &mut R, max: Option<U256>) -> Self {
        // The maximum value is the maximum between max and NodeIndex::MAX.
        let max_value = match max {
            Some(m) if m < EdgePath::MAX.0 => m,
            _ => NodeIndex::MAX.0,
        };

        Self::new(get_random_u256(U256::ONE, max_value + 1))
    }
}

impl RandomValue for FilledNode<LeafDataImpl> {
    fn random<R: Rng>(rng: &mut R, max: Option<U256>) -> Self {
        Self {
            data: NodeData::random(rng, max),
            hash: HashOutput::random(rng, max),
        }
    }
}

impl DumpyRandomValue for FilledTreeImpl {
    /// Generates a dumpy random filled tree.
    /// The tree contains up to max(m,101) random nodes in random indexes.
    /// Do not necessary represent a valid tree.
    fn dumpy_random<R: Rng>(rng: &mut R, max_size: Option<U256>) -> Self {
        // The maximum node number is the maximum between max and 101.
        let max_node_number = match max_size {
            Some(m) => m,
            _ => U256::from(101_u8),
        }
        .as_usize();

        let mut nodes: Vec<(NodeIndex, FilledNode<LeafDataImpl>)> = (0..max_node_number)
            .map(|_| {
                (
                    NodeIndex::random(rng, max_size),
                    FilledNode::random(rng, max_size),
                )
            })
            .collect();

        nodes.push((NodeIndex::ROOT, FilledNode::random(rng, max_size)));

        Self {
            tree_map: nodes.into_iter().collect(),
        }
    }
}

impl DumpyRandomValue for FilledForestImpl {
    /// Generates a dumpy random filled forest.
    /// The forest contains max(m,101) dumpy random storage trees,
    /// a dumpy random contract tree and a dumpy random compiled class tree.
    /// Does not necessary represent a valid forest.
    fn dumpy_random<R: Rng>(rng: &mut R, max_size: Option<U256>) -> Self {
        // The maximum storage trees number is the maximum between max and 98.
        // We also use this number to be the maximum tree size,
        let max_trees_number = match max_size {
            Some(m) => m,
            _ => U256::from(98_u8),
        }
        .as_usize();

        let storage_trees: HashMap<NodeIndex, FilledTreeImpl> = (0..max_trees_number)
            .map(|_| {
                (
                    NodeIndex::random(rng, max_size),
                    FilledTreeImpl::dumpy_random(rng, max_size),
                )
            })
            .collect::<HashMap<_, _>>();

        let contract_tree = FilledTreeImpl::dumpy_random(rng, max_size);
        let compiled_class_tree = FilledTreeImpl::dumpy_random(rng, max_size);

        Self {
            storage_trees,
            contract_tree,
            compiled_class_tree,
        }
    }
}
