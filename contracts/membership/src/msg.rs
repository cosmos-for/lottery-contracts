use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub title: String,
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
}

#[cw_serde]
pub struct AgentListsResp {
    pub agents: Vec<Addr>,
}

