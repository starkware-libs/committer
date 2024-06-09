use rstest::{fixture, rstest};

use crate::patricia_merkle_tree::node_data::inner_node::PathToBottom;
use crate::patricia_merkle_tree::node_data::leaf::{LeafModifications, SkeletonLeaf};
use crate::patricia_merkle_tree::original_skeleton_tree::node::OriginalSkeletonNode;
use crate::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTreeImpl;
use crate::patricia_merkle_tree::test_utils::get_initial_updated_skeleton;
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::patricia_merkle_tree::updated_skeleton_tree::node::UpdatedSkeletonNode;
use crate::patricia_merkle_tree::updated_skeleton_tree::tree::{
    UpdatedSkeletonTree, UpdatedSkeletonTreeImpl,
};

#[fixture]
fn initial_updated_skeleton(
    #[default(&[])] original_skeleton: &[(NodeIndex, OriginalSkeletonNode)],
    #[default(&[])] leaf_modifications: &[(NodeIndex, u8)],
) -> UpdatedSkeletonTreeImpl {
    get_initial_updated_skeleton(original_skeleton, leaf_modifications)
}

#[rstest]
#[should_panic]
#[case::empty_to_empty_illegal_modifications(&[], &[(NodeIndex::FIRST_LEAF, 0)], &[])]
#[case::empty_to_edge(
    &[],
    &[(NodeIndex::FIRST_LEAF, 1)],
    &[(NodeIndex::ROOT, UpdatedSkeletonNode::Edge(PathToBottom::from("0".repeat(251).as_str())))],
)]
#[case::empty_to_binary(
    &[],
    &[(NodeIndex::FIRST_LEAF, 1), (NodeIndex::FIRST_LEAF + 1, 1)],
    &([
        (NodeIndex::FIRST_LEAF >> 1, UpdatedSkeletonNode::Binary),
        (NodeIndex::ROOT, UpdatedSkeletonNode::Edge(PathToBottom::from("0".repeat(250).as_str()))),
    ]),
)]
#[case::nonempty_to_empty_tree(
    &[
        (NodeIndex::ROOT,
        OriginalSkeletonNode::Edge(PathToBottom::from("0".repeat(251).as_str())),
    )],
    &[(NodeIndex::FIRST_LEAF, 0)],
    &[]
)]
#[case::non_empty_to_binary(
    &[
        (NodeIndex::ROOT,
        OriginalSkeletonNode::Edge(PathToBottom::from("0".repeat(251).as_str())),
    )],
    &[
        (NodeIndex::FIRST_LEAF, 1),
        (NodeIndex::FIRST_LEAF + 1, 1)
    ],
    &[
        (NodeIndex::ROOT, UpdatedSkeletonNode::Edge(PathToBottom::from("0".repeat(250).as_str()))),
        (NodeIndex::FIRST_LEAF >> 1, UpdatedSkeletonNode::Binary)
    ]

)]
#[case::non_empty_replace_edge_bottom(
    &[
        (NodeIndex::ROOT,
        OriginalSkeletonNode::Edge(PathToBottom::from("0".repeat(251).as_str())),
    )],
    &[
        (NodeIndex::FIRST_LEAF, 0),
        (NodeIndex::FIRST_LEAF + 1, 1)
    ],
    &[
        (NodeIndex::ROOT, UpdatedSkeletonNode::Edge(PathToBottom::from(("0".repeat(250)+"1").as_str()))),
    ]

)]

fn test_updated_skeleton_tree_impl_create(
    #[case] original_skeleton: &[(NodeIndex, OriginalSkeletonNode)],
    #[case] leaf_modifications: &[(NodeIndex, u8)],
    #[case] expected_skeleton_additions: &[(NodeIndex, UpdatedSkeletonNode)],
    #[with(original_skeleton, leaf_modifications)]
    initial_updated_skeleton: UpdatedSkeletonTreeImpl,
) {
    let mut original_skeleton = OriginalSkeletonTreeImpl {
        nodes: original_skeleton.iter().cloned().collect(),
    };
    let leaf_modifications: LeafModifications<SkeletonLeaf> = leaf_modifications
        .iter()
        .map(|(index, val)| (*index, (*val).into()))
        .collect();
    let updated_skeleton_tree =
        UpdatedSkeletonTreeImpl::create(&mut original_skeleton, &leaf_modifications).unwrap();

    let mut expected_skeleton_tree = initial_updated_skeleton.skeleton_tree.clone();
    expected_skeleton_tree.extend(expected_skeleton_additions.iter().cloned());

    assert_eq!(updated_skeleton_tree.skeleton_tree, expected_skeleton_tree);
}
