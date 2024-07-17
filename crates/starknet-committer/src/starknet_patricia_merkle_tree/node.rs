use committer::patricia_merkle_tree::types::NodeIndex;
use starknet_types_core::felt::FromStrError;
use committer::felt::Felt;
use committer::impl_from_hex_for_felt_wrapper;

use crate::block_committer::input::ContractAddress;

// TODO(Nimrod, 1/6/2024): Use the ClassHash defined in starknet-types-core when available.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ClassHash(pub Felt);

impl_from_hex_for_felt_wrapper!(ClassHash);
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Nonce(pub Felt);

impl_from_hex_for_felt_wrapper!(Nonce);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CompiledClassHash(pub Felt);

impl_from_hex_for_felt_wrapper!(CompiledClassHash);

pub(crate) fn from_contract_address(address: &ContractAddress) -> NodeIndex {
        NodeIndex::from_leaf_felt(&address.0)
    }
