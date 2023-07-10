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
// User agents list created
pub const AGENTS: Map<&Addr, u64> = Map::new("agents");

