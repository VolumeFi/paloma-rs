use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint256};

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Store { order: Order },
}

#[cw_serde]
pub struct OrderWithSender {
    pub sender: Addr,
    pub order: Order,
}

#[cw_serde]
pub struct Order {
    pub token0: String,
    pub token1: String,
    pub amount0: Uint256,
    pub amount1: Uint256,
    pub depositor: Addr,
    pub deposit_price: f64,
    pub tracking_price: f64,
    pub profit_taking: u8,
    pub stop_loss: u8,
    pub withdraw_type: u8,
    pub withdraw_amount: Uint256,
    pub withdrawer: String,
    pub network_name: String,
}

#[cw_serde]
pub enum QueryMsg {
    LatestDepositId,
    Orders { deposit_id: u64, count: u64 },
}
