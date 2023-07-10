pub mod exec;
pub mod query;
pub mod reply;

use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;
use cw_storage_plus::Item;

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{Config, AGENTS, AGENT_MAP, CONFIG},
    ContractError,
};

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CREATED_AGENT_REPLY_ID: u64 = 1;
pub const CLOSE_LOTTERY_REPLY_ID: u64 = 2;

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
    state_item: Item<Config>,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let height = env.block.height;

    let config = Config {
        title: msg.title,
        height,
        owner: info.sender,
        agent_code_id: msg.agent_code_id,
        counter: 0,
    };

    state_item.save(deps.storage, &config)?;

    Ok(Response::new())
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        CreateAgent { name } => exec::create_agent(deps, env, info, name, CONFIG),
    }
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        AgentLists {} => query::agent_lists(deps, AGENTS).and_then(|resp| to_binary(&resp)),
        CurrentConfig {} => query::current_config(deps, CONFIG).and_then(|resp| to_binary(&resp)),
    }
}

pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        CREATED_AGENT_REPLY_ID => reply::created_agent(
            deps,
            env,
            reply.result.into_result(),
            AGENTS,
            CONFIG,
            AGENT_MAP,
        ),
        id => Err(ContractError::UnRecognizedReplyIdErr { id }),
    }
}
