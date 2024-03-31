use pathfinder_crypto::Felt;
use rand::Rng;
use starknet_types_core::felt::Felt as StarknetFelt;
use std::{collections::HashMap, time::Duration};

use crate::test::SNTreeNode;

pub fn mean(data: &[Duration]) -> Duration {
    data.iter()
        .sum::<Duration>()
        .checked_div(data.len().try_into().unwrap())
        .unwrap()
}

#[allow(clippy::as_conversions)]
pub fn std_deviation(data: &[Duration]) -> Duration {
    let mean = mean(data).as_secs_f32();
    let mut variance = data
        .iter()
        .map(|x| {
            let diff = (*x).as_secs_f32() - mean;
            diff * diff
        })
        .sum::<f32>();
    variance /= data.len() as f32;
    Duration::from_secs_f32(variance.sqrt())
}

pub fn random_felt() -> StarknetFelt {
    let mut buf: [u8; 32] = rand::thread_rng().gen();
    buf[0] &= 0x07; // clear the 5 most significant bits
    StarknetFelt::from_bytes_be(&buf)
}

pub fn tree_to_hashmap(root: SNTreeNode) -> HashMap<StarknetFelt, SNTreeNode> {
    let mut map = HashMap::new();
    tree_to_hashmap_rec(root, &mut map, ONE);
    map
}

// TODO: handle edge nodes
pub fn tree_to_hashmap_rec(
    node: SNTreeNode,
    map: &mut HashMap<StarknetFelt, SNTreeNode>,
    index: StarknetFelt,
) {
    map.insert(
        index,
        SNTreeNode {
            left_child: None,
            right_child: None,
            _parent: None,
            hash_value: node.hash_value,
            is_path: node.is_path,
            _path_data: node._path_data,
            _is_root: node._is_root,
            is_leaf: node.is_leaf,
            length: node.length,
        },
    );

    //node.clone());
    if let Some(left_child) = node.left_child {
        tree_to_hashmap_rec(*left_child, map, index * TWO);
    }
    if let Some(right_child) = node.right_child {
        tree_to_hashmap_rec(*right_child, map, index * TWO + ONE);
    }
}

pub fn get_left_child(index: u128, map: &HashMap<Felt, SNTreeNode>) -> Option<&SNTreeNode> {
    let left_child_index = index * 2;
    map.get(&Felt::from(left_child_index))
}

pub const TWO: StarknetFelt = StarknetFelt::TWO;
pub const ONE: StarknetFelt = StarknetFelt::ONE;
pub const TREE_HEIGHT: u8 = 15;
pub fn create_dummy_tree(height: u8) -> Option<Box<SNTreeNode>> {
    let node = match height {
        TREE_HEIGHT => SNTreeNode {
            left_child: create_dummy_tree(height - 1),
            right_child: create_dummy_tree(height - 1),
            _parent: None,
            hash_value: None,
            is_path: false,
            _path_data: None,
            _is_root: true,
            is_leaf: false,
            length: 0,
        },
        0 => SNTreeNode {
            left_child: None,
            right_child: None,
            _parent: None,
            hash_value: Some(random_felt()),
            is_path: false,
            _path_data: None,
            _is_root: false,
            is_leaf: true,
            length: 0,
        },
        _ => SNTreeNode {
            left_child: create_dummy_tree(height - 1),
            right_child: create_dummy_tree(height - 1),
            _parent: None,
            hash_value: None,
            is_path: false,
            _path_data: None,
            _is_root: false,
            is_leaf: false,
            length: 0,
        },
    };
    Some(Box::new(node))
}

pub fn print_tree(node: &Option<Box<SNTreeNode>>) {
    match node {
        Some(node) => {
            println!("{:?}", node.hash_value);
            print_tree(&node.left_child);
            print_tree(&node.right_child);
        }
        None => {}
    }
}
pub fn clone_tree(node: &Option<Box<SNTreeNode>>) -> Option<Box<SNTreeNode>> {
    match node {
        Some(node) => {
            let mut new_node = SNTreeNode {
                left_child: None,
                right_child: None,
                _parent: None,
                hash_value: node.hash_value,
                is_path: node.is_path,
                _path_data: node._path_data,
                _is_root: node._is_root,
                is_leaf: node.is_leaf,
                length: node.length,
            };
            new_node.left_child = clone_tree(&node.left_child);
            new_node.right_child = clone_tree(&node.right_child);
            Some(Box::new(new_node))
        }
        None => None,
    }
    // match node {
    //     Some(node) => {
    //         let mut new_node = node.clone();
    //         new_node.left_child = clone_tree(&node.left_child);
    //         new_node.right_child = clone_tree(&node.right_child);
    //         Some(new_node)
    //     }
    //     None => None,
    // }
}

pub fn count_inner_nodes(node: &Option<Box<SNTreeNode>>) -> u32 {
    match node {
        Some(node) => {
            let mut count = 0;
            if !node.is_leaf {
                count += 1;
                count += count_inner_nodes(&node.left_child);
                count += count_inner_nodes(&node.right_child);
            }
            count
        }
        None => 0,
    }
}
