pub mod exec;
pub mod query;
pub mod reply;

use cosmwasm_std::{DepsMut, Env, MessageInfo, Reply, Response};
use cw2::set_contract_version;
use cw_storage_plus::Item;

use crate::{
    msg::{ExecuteMsg, InstantiateMsg},
    state::{State, BETTORS, STATE, WITHDRAWS},
    ContractError,
};

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
    state_item: Item<State>,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let blockheight = env.block.height;

    let state = State {
        version: 1,
        height: blockheight,
        title: msg.title,
        owner: info.sender,
        rewards: vec![],
        winner: None,
    };

    state_item.save(deps.storage, &state)?;

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
        Buy { denom, memo } => exec::buy(deps, env, info, memo, denom, STATE, BETTORS),
        Draw {} => exec::draw(deps, env, info, STATE, BETTORS),
        WithdrawRewards { amount, denom } => {
            exec::withdraw(deps, env, info, amount, denom.as_str(), STATE, WITHDRAWS)
        }
    }
}
pub fn reply(_deps: DepsMut, _env: Env, _reply: Reply) -> Result<Response, ContractError> {
    todo!()
}
