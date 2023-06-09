use crate::msg::OrderWithSender;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const ADMIN: Item<Addr> = Item::new("admin");
pub const DEPOSIT_ID_SEQ: Item<u64> = Item::new("deposit_id_seq");
pub const EXECUTED_DEPOSIT_ID: Item<u64> = Item::new("executed_deposit_id");

pub const DEPOSITS: Map<u64, OrderWithSender> = Map::new("deposits");
