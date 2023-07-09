use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub title: String,
}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(LotteriesCountResp)]
    LotteriesCount {},
    #[returns(LatestLotteryResp)]
    LatestLottery {},
}

#[cw_serde]
pub struct LotteriesCountResp {
    pub counter: u64,
}

#[cw_serde]
pub struct InstantiationData {
    pub addr: Addr,
}

#[cw_serde]
pub struct LatestLotteryResp {
    pub lottery: Option<Addr>,
}
