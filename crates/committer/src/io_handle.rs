use pathfinder_crypto::Felt;
use serde::Deserialize;
use std::fs::File;

use crate::patricia_merkle_tree::node::{IndexedNode, NodeIndex, TreeNode};
#[allow(dead_code)]

pub(super) fn parse_input_file(input_path: String) -> Vec<Vec<IndexedNode>> {
    let reader = File::open(input_path).unwrap();
    let deseralized_nodes_by_dependency_layer: Vec<Vec<DeserializedIndexedNode>> =
        serde_json::from_reader(reader).unwrap();
    let nodes_by_dependency_layer: Vec<Vec<IndexedNode>> = deseralized_nodes_by_dependency_layer
        .iter()
        .map(|inner_vec| {
            inner_vec
                .iter()
                .map(|deseralized_node| deseralized_node.to_indexed_node())
                .collect()
        })
        .collect();
    print!("{:?}", nodes_by_dependency_layer);
    nodes_by_dependency_layer
}

// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where
//     P: AsRef<Path>,
// {
//     let file = File::open(filename)?;
//     Ok(io::BufRead::lines(io::BufReader::new(file)))
// }
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct DeserializedIndexedNode {
    // index_as_array: Vec<u8>,
    index: u128,
    node: DeserializedNode,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct DeserializedNode {
    // TODO(Nimrod, 5/4/2024): path should be felt as well.
    node_type: u8,
    length: u8,
    path: Vec<u8>,
    value: Vec<u8>,
}

impl DeserializedIndexedNode {
    pub(crate) fn to_indexed_node(&self) -> IndexedNode {
        // TODO(Nimrod, 10/4/2024): Where should this values be? need to match python's hard coded
        // values.
        let binary_node_type = 1;
        let edge_node_type = 2;
        let leaf_node_type = 3;

        if self.node.node_type == binary_node_type {
            return (
                NodeIndex(Felt::from_u128(self.index)),
                TreeNode::Binary {
                    left: Felt::from_u128(0),
                    right: Felt::from_u128(0),
                },
            );
        }
        if self.node.node_type == edge_node_type {
            (
                NodeIndex(Felt::from_u128(self.index)),
                TreeNode::Edge {
                    length: self.node.length,
                    path: Felt::from_be_slice(&self.node.path).unwrap(),
                    value: Felt::from_be_slice(&self.node.value).unwrap(),
                },
            )
        } else {
            assert_eq!(self.node.node_type, leaf_node_type);
            (
                NodeIndex(Felt::from_u128(self.index)),
                TreeNode::Leaf {
                    value: Felt::from_be_slice(&self.node.value).unwrap(),
                },
            )
        }
    }
}
