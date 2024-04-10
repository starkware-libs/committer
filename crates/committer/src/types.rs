use starknet_types_core::felt::Felt as StarknetTypesFelt;

pub(crate) type Felt = StarknetTypesFelt;

pub(crate) trait FeltTrait {
    fn to_bytes(&self) -> [u8; 32];
}


impl FeltTrait for Felt {
    fn to_bytes(&self) -> [u8; 32] {
        self.to_bytes_be()
    }

}