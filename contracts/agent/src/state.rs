use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct State {
    pub name: String,
    pub height: u64,
    pub created_at: u64,
    pub counter: u64,
    pub owner: Addr,
}

#[cw_serde]
pub struct BetInfo {
    pub buy_at: u64,
    pub memo: Option<String>,
}

#[cw_serde]
pub struct LotteryInfo {
    pub lottery: Addr,
    pub rewards: Vec<Coin>,
}

pub const STATE: Item<State> = Item::new("state");

pub const LOTTERIES_WINNED: Map<&Addr, LotteryInfo> = Map::new("lotteries_winned");

pub const LOTTERIES_JOINED: Item<Vec<Addr>> = Item::new("lotteries_joined");

pub const LOTTERIES_JOINED_MAP: Map<&Addr, BetInfo> = Map::new("lotteries_joined_map");
