use cosmwasm_std::{Addr, Deps, StdResult};
use cw_storage_plus::Item;

use crate::{
    msg::{AgentListsResp, CurrentConfigResp},
    state::Config,
};

pub fn agent_lists(deps: Deps, agents_item: Item<Vec<Addr>>) -> StdResult<AgentListsResp> {
    let agents = agents_item.may_load(deps.storage)?;

    Ok(AgentListsResp {
        agents: agents.unwrap_or_default(),
    })
}

pub fn current_config(deps: Deps, config: Item<Config>) -> StdResult<CurrentConfigResp> {
    let config = config.load(deps.storage)?;
    Ok(CurrentConfigResp { config })
}
