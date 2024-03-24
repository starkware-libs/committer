use std::env;
use std::path::Path;
// use std::process::Output;
use rand::Rng;
use std::time::Instant;

use async_recursion::async_recursion;
use pathfinder_crypto::Felt;

const TREE_HEIGHT: u8 = 18;

/// Main entry point of the committer CLI.
#[tokio::main]
async fn main() {
    let real_start = Instant::now();
    // Open the input file.
    let args: Vec<String> = env::args().collect();
    let input_file_path = Path::new(&args[1]);
    let output_file_path = Path::new(&args[2]);
    assert!(
        input_file_path.is_absolute() && output_file_path.is_absolute(),
        "Given paths must be absolute"
    );
    // let input_file_name = input_file_path.file_name().unwrap().to_str().unwrap();

    // Business logic to be implemented here.
    // initialize the tree
    let _height = TREE_HEIGHT;
    // fill the tree with data
    let now = Instant::now();
    let root = create_dummy_tree(_height);
    let root_clone = clone_tree(&root);
    let elapased_time = now.elapsed();
    println!("Tree creation time: {:?}", elapased_time);

    let now = Instant::now();
    // Code block to measure.
    let result_tokio = algorithm_tokio(*root_clone.unwrap()).await;
    // TODO: sanity check
    // End of code block to measure.
    let elapased_time_tokio = now.elapsed();
    // Print measurement.
    println!("Tokio time: {:?}", elapased_time_tokio);

    let now = Instant::now();
    // Code block to measure.
    let result_seq = algorithm_seq(*root.unwrap());
    // TODO: sanity check
    // End of code block to measure.
    let elapased_time_seq = now.elapsed();
    // Print measurement.
    println!("Sequential time: {:?}", elapased_time_seq);

    // Sanity check.
    assert_eq!(result_seq, result_tokio);
    println!("Sanity check passed!");
    println!(
        "Tokio took {}% less runtime",
        100_f64 * (elapased_time_seq - elapased_time_tokio).as_secs_f64()
            / elapased_time_seq.as_secs_f64()
    );

    // Output to file.
    let output = "Dummy output";
    std::fs::write(output_file_path, output).expect("Failed to write output");
    println!("Total time: {:?}", real_start.elapsed());
}

pub struct SNTreeNode {
    left_child: Option<Box<SNTreeNode>>,
    right_child: Option<Box<SNTreeNode>>,
    _parent: Option<Box<SNTreeNode>>,
    hash_value: Option<Felt>,
    is_path: bool,
    _path_data: Option<[u128; 2]>,
    _is_root: bool,
    is_leaf: bool,
    // _leaf_data: Option<(ClassHash, ClassTreeNode)>,
}

pub fn random_felt() -> Felt {
    let mut buf: [u8; 32] = rand::thread_rng().gen();
    buf[0] &= 0x01; // clear the 4 most significant bits
    Felt::from_be_bytes(buf).expect("Overflow ;(")
}

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
        },
    };
    Some(Box::new(node))
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
            };
            new_node.left_child = clone_tree(&node.left_child);
            new_node.right_child = clone_tree(&node.right_child);
            Some(Box::new(new_node))
        }
        None => None,
    }
}

pub async fn compute_val(node: SNTreeNode) -> Felt {
    match node.hash_value {
        Some(value) => value,
        None => algorithm_tokio(node).await,
    }
}

#[async_recursion]
pub async fn algorithm_tokio(mut node: SNTreeNode) -> Felt {
    if node.is_leaf {
        //TODO: compute/return the leaf hash
        return node.hash_value.unwrap();
    }
    let left_child = node
        .left_child
        .expect("Not a leaf node, left child must exist");
    let right_child = node
        .right_child
        .expect("Not a leaf node, right child must exist");
    let left_value_future = tokio::spawn(compute_val(*left_child));
    let right_value_future = tokio::spawn(compute_val(*right_child));
    if !node.is_path {
        let (left_value, right_value) = (
            left_value_future.await.unwrap(),
            right_value_future.await.unwrap(),
        );
        node.hash_value = Some(pathfinder_crypto::hash::pedersen_hash(
            left_value,
            right_value,
        ));
        return node.hash_value.unwrap();
    } else {
        //TODO: compute/return the path hash
        todo!("Path hash computation")
    }
}

pub fn algorithm_seq(mut node: SNTreeNode) -> Felt {
    if node.is_leaf {
        return node.hash_value.unwrap();
    }
    let left_child = node
        .left_child
        .expect("Not a leaf node, left child must exist");
    let right_child = node
        .right_child
        .expect("Not a leaf node, right child must exist");
    let left_value = algorithm_seq(*left_child);
    let right_value = algorithm_seq(*right_child);
    if !node.is_path {
        node.hash_value = Some(pathfinder_crypto::hash::pedersen_hash(
            left_value,
            right_value,
        ));
        node.hash_value.unwrap()
    } else {
        Felt::default()
    }
}
