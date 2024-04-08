use starknet_types_core::felt::Felt;

use crate::{
    patricia_merkle_tree::types::{EdgePath, EdgePathLength, NodeIndex, PathToBottom},
    types::{ONE, TWO},
};

use super::compute_bottom_index;

#[test]
fn test_compute_bottom_index() {
    let bottom_index = compute_bottom_index(
        NodeIndex(ONE),
        PathToBottom {
            path: EdgePath(ONE),
            length: EdgePathLength(1),
        },
    );
    assert_eq!(bottom_index, NodeIndex(Felt::THREE));
    let bottom_index = compute_bottom_index(
        NodeIndex(ONE),
        PathToBottom {
            path: EdgePath(Felt::ZERO),
            length: EdgePathLength(1),
        },
    );
    assert_eq!(bottom_index, NodeIndex(TWO));
    let bottom_index = compute_bottom_index(
        NodeIndex::root_index(),
        PathToBottom {
            path: EdgePath(Felt::ZERO),
            length: EdgePathLength(2),
        },
    );
    assert_eq!(bottom_index, NodeIndex(Felt::from(1 << 2)));
    let bottom_index = compute_bottom_index(
        NodeIndex(Felt::from(0xDAD)),
        PathToBottom {
            path: EdgePath(Felt::from(0xFEE)),
            length: EdgePathLength(12),
        },
    );
    assert_eq!(bottom_index, NodeIndex(Felt::from(0xDADFEE_u32)));
    let bottom_index = compute_bottom_index(
        NodeIndex(Felt::from(0xDEAFBEE_u32)),
        PathToBottom {
            path: EdgePath(Felt::from(0xBFF)),
            length: EdgePathLength(16),
        },
    );
    assert_eq!(bottom_index, NodeIndex(Felt::from(0xDEAFBEE0BFF_u64)));
    let bottom_index = compute_bottom_index(
        NodeIndex::root_index(),
        PathToBottom {
            path: EdgePath(Felt::ZERO),
            length: EdgePathLength(0),
        },
    );
    assert_eq!(bottom_index, NodeIndex::root_index());
}
