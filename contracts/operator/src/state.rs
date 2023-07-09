use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub title: String,
    pub owner: Addr,
    pub counter: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
// Lotteries list created
pub const LOTTERIES: Map<&Addr, u64> = Map::new("lotteries");

pub const LATEST_LOTTERY: Item<Addr> = Item::new("latest_lottery");
