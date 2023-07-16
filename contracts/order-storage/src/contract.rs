use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, OrderWithSender, QueryMsg};
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use eyre::Result;

use crate::state::{ADMIN, DEPOSITS, DEPOSIT_ID_SEQ, EXECUTED_DEPOSIT_ID};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::Order::Ascending;
use cw_storage_plus::Bound;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    ADMIN.save(deps.storage, &info.sender)?;
    DEPOSIT_ID_SEQ.save(deps.storage, &0)?;
    EXECUTED_DEPOSIT_ID.save(deps.storage, &0)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response> {
    let ExecuteMsg::Store { order } = msg;
    let deposit_id = DEPOSIT_ID_SEQ.update(deps.storage, |x| StdResult::Ok(x + 1))?;
    let order = OrderWithSender {
        sender: info.sender,
        order,
    };
    DEPOSITS.save(deps.storage, deposit_id, &order)?;
    Ok(Response::new().add_attribute("deposit_id", deposit_id.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;
    match msg {
        LatestDepositId => to_binary(&DEPOSIT_ID_SEQ.load(deps.storage)?),
        Orders { deposit_id, count } => {
            // for entry in REGISTER.range(deps.storage, None, None, Ascending) {
            //                 let (addr, amt) = entry?;
            //                 res = res.add_attribute(&addr, amt);
            //             }
            let orders: Vec<OrderWithSender> = DEPOSITS
                .range(
                    deps.storage,
                    Some(Bound::inclusive(deposit_id)),
                    None,
                    Ascending,
                )
                .take(count as usize)
                .map(|r| r.unwrap().1)
                .collect();
            to_binary(&orders)
        }
    }
}
