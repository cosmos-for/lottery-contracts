use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub title: String,
    pub agent_code_id: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateAgent { name: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AgentListsResp)]
    AgentLists {},
    #[returns(CurrentConfigResp)]
    CurrentConfig {},
}

#[cw_serde]
pub struct AgentListsResp {
    pub agents: Vec<Addr>,
}

#[cw_serde]
pub struct CurrentConfigResp {
    pub config: Config,
}

#[cw_serde]
pub struct InstantiationData {
    pub addr: Addr,
}
