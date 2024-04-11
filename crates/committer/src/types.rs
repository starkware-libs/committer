use starknet_types_core::felt::{Felt as StarknetTypesFelt, FromStrError};

#[derive(
    Eq,
    PartialEq,
    Clone,
    Copy,
    Debug,
    Default,
    Hash,
    derive_more::Add,
    PartialOrd,
    Ord,
    derive_more::Sub,
)]
pub(crate) struct Felt(StarknetTypesFelt);

#[macro_export]
macro_rules! impl_from {
    ($to:ty, $from:ty, $($other_from: ty),+) => {
        $crate::impl_from!($to, $from);
        $crate::impl_from!($to $(, $other_from)*);
    };
    ($to:ty, $from:ty) => {
        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                Self(value.into())
            }
        }
    };
}
impl_from!(Felt, StarknetTypesFelt, u128, u8);

impl From<Felt> for StarknetTypesFelt {
    fn from(felt: Felt) -> Self {
        felt.0
    }
}

impl std::ops::Mul for Felt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

#[allow(dead_code)]
impl Felt {
    pub const ZERO: Felt = Felt(StarknetTypesFelt::ZERO);
    pub const ONE: Felt = Felt(StarknetTypesFelt::ONE);
    pub const TWO: Felt = Felt(StarknetTypesFelt::TWO);

    /// Raises `self` to the power of `exponent`.
    pub fn pow(&self, exponent: impl Into<u128>) -> Self {
        Self(self.0.pow(exponent.into()))
    }
    pub fn bits(&self) -> u8 {
        self.0
            .bits()
            .try_into()
            .expect("Should not fail as it takes less than 252 bits to represent a felt.")
    }

    pub fn shift_left(&self, steps: u8) -> Self {
        *self * Felt::TWO.pow(steps)
    }
    pub fn from_bytes_be_slice(bytes: &[u8]) -> Self {
        Self(StarknetTypesFelt::from_bytes_be_slice(bytes))
    }
    pub(crate) fn to_bytes_be(self) -> [u8; 32] {
        self.0.to_bytes_be()
    }

    /// Parse a hex-encoded number into `Felt`.
    pub fn from_hex(hex_string: &str) -> Result<Self, FromStrError> {
        Ok(StarknetTypesFelt::from_hex(hex_string)?.into())
    }
}
