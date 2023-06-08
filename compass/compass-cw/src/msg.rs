use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Uint256};

#[cw_serde]
pub struct MigrateMsg {}

#[derive(Eq, PartialOrd, Ord)]
#[cw_serde]
pub struct ValsetId(pub Uint256);

#[cw_serde]
pub struct Valset {
    pub valset_id: ValsetId,
    pub validators: Vec<Binary>,
    pub powers: Vec<u32>,
}

#[cw_serde]
pub struct Signature(pub Vec<u8>);

#[cw_serde]
pub struct Consensus {
    /// Signatures must be in the same order as the validator array in `valset`
    pub signatures: Vec<Option<Signature>>,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub smart_contract_id: Addr,
    pub valset: Valset,
}

pub type MessageId = Uint256;

#[cw_serde]
pub struct ExecuteMsg {
    pub consensus: Consensus,
    pub payload: Binary,
}

#[cw_serde]
pub enum ExecutePayload {
    UpdateValset {
        valset: Valset,
        smart_contract_id: Addr,
    },
    SubmitLogicCall {
        logic_call_args: LogicCallArgs,
        message_id: MessageId,
        smart_contract_id: Addr,
        deadline: u64,
    },
}

#[cw_serde]
pub struct LogicCallArgs {
    pub contract_address: Addr,
    pub payload: String,
}

#[cw_serde]
pub enum QueryMsg {
    SmartContractId,
    ValsetId,
}
