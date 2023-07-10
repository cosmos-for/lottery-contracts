use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Empty};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub title: String,
    pub height: u64,
    pub owner: Addr,
    pub agent_code_id: u64,
    pub counter: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
// User agents list created
pub const AGENT_MAP: Map<&Addr, Empty> = Map::new("agent_map");

pub const AGENTS: Item<Vec<Addr>> = Item::new("agents");
