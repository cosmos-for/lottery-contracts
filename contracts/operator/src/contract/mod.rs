pub mod exec;
pub mod reply;

use cosmwasm_std::{DepsMut, Env, MessageInfo, Reply, Response};
use cw2::set_contract_version;
use cw_storage_plus::Item;

use crate::{
    msg::{ExecMsg, InstantiateMsg},
    state::{Config, LOTTERIES},
    ContractError,
};

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const INITIAL_LOTTERY_INSTANTIATION_REPLY_ID: u64 = 1;

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
    state_item: Item<Config>,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        title: msg.title,
        owner: info.sender,
    };

    state_item.save(deps.storage, &config)?;

    Ok(Response::new())
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecMsg,
) -> Result<Response, ContractError> {
    todo!()
}

pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        INITIAL_LOTTERY_INSTANTIATION_REPLY_ID => {
            reply::initial_lottery_instantiate(deps, env, reply.result.into_result(), LOTTERIES)
        }
        id => Err(ContractError::UnRecognizedReplyIdErr { id }),
    }
}
