use pathfinder_crypto::Felt;
use patricia_merkle_tree::{
    hash::{HashInput, HashOutput},
    node::NodeIndex,
};

pub mod patricia_merkle_tree;

// TODO(Dori, 3/3/2024): Delete this dummy code once rust allows it.
pub fn dummy() -> Felt {
    let felt = Felt::from_u64(0_u64);
    let hash_input = HashInput(felt, felt, felt);
    let hash_output = HashOutput(felt);
    let node_index = NodeIndex(felt);
    // Rust requires that every field will be used.
    hash_input.0 + hash_input.1 + hash_input.2 + hash_output.0 + node_index.0
}

#[cfg(test)]
pub mod test {
    use super::dummy;
    use pathfinder_crypto::Felt;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_dummy() {
        assert_eq!(dummy(), Felt::from_u64(0_u64));
    }
}
