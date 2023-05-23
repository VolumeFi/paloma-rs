use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use cosmwasm_std::{
    to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw_storage_plus::Item;
use eyre::Result;
use xcci::ExecutePalomaJob;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new())
}

pub const ADMIN: Item<Addr> = Item::new("admin");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    if msg.admin {
        ADMIN.save(deps.storage, &info.sender)?;
    }
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ExecutePalomaJob>> {
    if let Some(addr) = ADMIN.may_load(deps.storage)? {
        assert_eq!(addr, info.sender, "Incorrect sender");
    }
    let ExecuteMsg::Call {
        target_contract_info,
        payload,
    } = msg;
    Ok(
        Response::new().add_message(CosmosMsg::Custom(ExecutePalomaJob {
            target_contract_info,
            payload,
        })),
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    to_binary(&())
}
