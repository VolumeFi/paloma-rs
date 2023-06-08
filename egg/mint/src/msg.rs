use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;
use xcci::TargetContractInfo;

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct InstantiateMsg {
    pub target_contract_info: TargetContractInfo,
}

#[cw_serde]
pub enum ExecuteMsg {
    LayEgg { eth_address: String },
    PickWinner { payload: Binary },
}

#[cw_serde]
pub enum QueryMsg {}
