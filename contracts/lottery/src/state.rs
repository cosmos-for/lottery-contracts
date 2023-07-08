use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct State {
    pub version: u64,
    // 彩票创建时的block height
    pub height: u64,
    pub title: String,
    // 彩票的所有者，彩票被 领奖 之前为 Operator，领奖后 为 中奖者
    pub owner: Addr,
    // 彩票的奖金，在开奖前为0，开奖时设置为实际的金额
    pub rewards: u64,

    pub winner: Option<Addr>,
}

#[cw_serde]
pub struct BetInfo {
    pub buy_at: u64,
    pub memo: Option<String>,
}

pub const STATE: Item<State> = Item::new("state");

pub const BETTORS: Map<&Addr, BetInfo> = Map::new("bettors");
