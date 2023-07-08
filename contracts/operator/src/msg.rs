use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub title: String,
}

#[cw_serde]
pub enum ExecMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryLotteriesResp)]
    QueryLotteries {},
}

#[cw_serde]
pub struct QueryLotteriesResp {
    pub lotteries: Vec<Addr>,
}

#[cw_serde]
pub struct InstantiationData {
    pub addr: Addr,
}
