use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct InstantiateMsg {
    pub delegate_address: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    Register { tx_meta: TxMeta },
    Claim {},
    WithdrawAll {},
}

#[cw_serde]
pub enum QueryMsg {
    Claim { address: Addr },
    ClaimedRewards { address: Addr },
    TotalClaimed {},
    TotalRegistered {},
}

#[cw_serde]
pub struct TxMeta {
    pub address: Addr,   // user paloma wallet address
    pub amount: Uint128, // register amount
    pub tx_hash: String, // swap transaction hash
}
