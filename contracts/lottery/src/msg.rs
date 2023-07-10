use crate::state::BetInfo;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct InstantiateMsg {
    pub title: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Buy {
        // addr: String,
        denom: String,
        memo: Option<String>,
    },
    Close {
        rewards: Vec<Coin>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(OwnerResp)]
    Owner {},
    #[returns(WinnerResp)]
    Winner {},
    #[returns(QueryBettorResp)]
    QueryBettor { bettor: String },
}

#[cw_serde]
pub struct OwnerResp {
    pub owner: Addr,
}

#[cw_serde]
pub struct WinnerResp {
    pub winner: Option<Addr>,
}

#[cw_serde]
pub struct QueryBettorResp {
    pub info: Option<BetInfo>,
}
