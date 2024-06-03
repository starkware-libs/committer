use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use starknet_api::block::GasPricePerToken;
use starknet_api::transaction::{Event, Fee, MessageToL1, TransactionExecutionStatus};

#[derive(Debug, Deserialize)]
pub(crate) struct BlockInfo {
    pub da_mode: bool,
    pub l1_gas_price_per_token: GasPricePerToken,
    pub l1_data_gas_price_per_token: GasPricePerToken,
}

pub type RawResourcesMapping = HashMap<String, u128>;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GasVector {
    pub l1_gas: u128,
    pub l1_data_gas: u128,
}

/// Stripped down `TransactionExecutionInfo` for Python serialization, containing only the required
/// fields.
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ThinTransactionExecutionInfo {
    pub events: Vec<Event>,
    pub l2_to_l1_messages: Vec<MessageToL1>,
    pub actual_fee: Fee,
    pub actual_resources: RawResourcesMapping,
    pub execution_status: TransactionExecutionStatus,
    pub da_gas: Option<GasVector>,
}
