use crate::state::{LotteryInfo, State};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub owner: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Buy {
        addr: String,
        denom: String,
        memo: Option<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(LotteriesJoinedResp)]
    LotteriesJoined {},
    #[returns(LotteriesWinnedResp)]
    LotteriesWinned {},
    #[returns(CurrentStateResp)]
    CurrentState {},
}

#[cw_serde]
pub struct LotteriesJoinedResp {
    pub lotteries: Vec<Addr>,
}
#[cw_serde]
pub struct LotteriesWinnedResp {
    pub lotteries: Vec<LotteryInfo>,
}

#[cw_serde]
pub struct CurrentStateResp {
    pub state: State,
}

#[cw_serde]
pub struct Buy {
    pub denom: String,
    pub memo: Option<String>,
}
