use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub struct MigrateMsg {}

fn truth() -> bool {
    true
}

#[cw_serde]
pub struct InstantiateMsg {
    #[serde(default = "truth")]
    pub admin: bool,
}

#[cw_serde]
pub enum ExecuteMsg {
    Call { job_id: String, payload: Binary },
}

#[cw_serde]
pub enum QueryMsg {}
