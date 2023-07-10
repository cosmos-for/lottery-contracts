use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct InstantiateMsg {
    pub title: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateLottery { lottery_code_id: u64, title: String },
    CloseLottery { lottery: String, rewards: Vec<Coin> },
    DrawLottery { lottery: String },
}

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
