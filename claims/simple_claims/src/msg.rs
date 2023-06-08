use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct InstantiateMsg {
    pub claims: Vec<(Addr, Uint128)>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Claim {},
    Clear {},
}

#[cw_serde]
pub enum QueryMsg {}
