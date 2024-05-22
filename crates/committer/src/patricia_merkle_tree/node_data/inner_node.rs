use crate::felt::Felt;
use crate::hash::hash_trait::HashOutput;
use crate::patricia_merkle_tree::node_data::leaf::LeafData;
use crate::patricia_merkle_tree::types::NodeIndex;

#[derive(Clone, Debug, PartialEq, Eq)]
// A Patricia-Merkle tree node's data, i.e., the pre-image of its hash.
pub enum NodeData<L: LeafData> {
    Binary(BinaryData),
    Edge(EdgeData),
    Leaf(L),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BinaryData {
    pub left_hash: HashOutput,
    pub right_hash: HashOutput,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct EdgePath(pub Felt);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct EdgePathLength(pub u8);

impl std::ops::Add for EdgePathLength {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PathToBottom {
    pub path: EdgePath,
    pub length: EdgePathLength,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct EdgeData {
    pub bottom_hash: HashOutput,
    pub path_to_bottom: PathToBottom,
}

impl PathToBottom {
    #[cfg(test)]
    pub(crate) const LEFT_CHILD: Self = PathToBottom {
        path: EdgePath(Felt::ZERO),
        length: EdgePathLength(1),
    };
    #[cfg(test)]
    pub(crate) const RIGHT_CHILD: Self = PathToBottom {
        path: EdgePath(Felt::ONE),
        length: EdgePathLength(1),
    };

    pub(crate) fn bottom_index(&self, root_index: NodeIndex) -> NodeIndex {
        NodeIndex::compute_bottom_index(root_index, self)
    }

    pub(crate) fn concat_paths(&self, other: PathToBottom) -> PathToBottom {
        PathToBottom {
            path: EdgePath((self.path.0 * Felt::TWO.pow(other.length.0)) + other.path.0),
            length: self.length + other.length,
        }
    }
}
