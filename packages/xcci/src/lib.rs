//! This crate implements the foreign chain call interface for Paloma.
//! Types here may be used to issue an event which initiates a call on
//! another blockchain. Example:
//!
//! ```rust
//! use cosmwasm_std::{Binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError, entry_point};
//! use xcci::{ExecutePalomaJob, TargetContractInfo};
//!
//! pub enum ExecuteMsg {
//!     Call {
//!         target_contract_info: TargetContractInfo,
//!         payload: Binary,
//!     }
//! }
//!
//! #[entry_point]
//! pub fn execute(
//!     _deps: DepsMut,
//!     _env: Env,
//!     _info: MessageInfo,
//!     msg: ExecuteMsg,
//! ) -> Result<Response<ExecutePalomaJob>, StdError> {
//!     let ExecuteMsg::Call {
//!         target_contract_info,
//!         payload,
//!     } = msg;
//!     Ok(
//!         Response::new().add_message(CosmosMsg::Custom(ExecutePalomaJob {
//!             target_contract_info,
//!             payload,
//!         })),
//!     )
//! }
//! ```

#![deny(missing_docs)]

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, CustomMsg};

/// The `CustomMsg` understood by Paloma to execute cross chain calls.
#[derive(Eq)]
#[cw_serde]
pub struct ExecuteJobWasmEvent {
    /// ID of the metadata of the foreign contract we wish to call.
    pub job_id: String,
    /// Payload for the call, encoded appropriately for the target chain and contract.
    pub payload: Binary,
}

impl CustomMsg for ExecuteJobWasmEvent {}
