use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct InstantiateMsg {
    pub job_id: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    LayEgg { eth_address: String },
    PickWinner { payload: Binary },
}

#[cw_serde]
pub enum QueryMsg {}
