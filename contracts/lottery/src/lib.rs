pub mod contract;
mod error;
pub mod helpers;
pub mod msg;
pub mod state;

#[cfg(any(feature = "mt", test))]
pub mod multitest;

pub use crate::error::ContractError;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use state::{BETTORS, STATE};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

pub const NATIVE_DENOM: &str = "LOTTERY";
pub const LOTTERY_FEE: u128 = 100;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    contract::instantiate(deps, env, info, msg, STATE)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Buy { denom, memo } => {
            contract::exec::buy(deps, env, info, memo, denom, STATE, BETTORS)
        }
        ExecuteMsg::Close { rewards } => {
            contract::exec::close(deps, env, info, rewards, STATE, BETTORS)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    contract::reply(deps, env, reply)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Owner {} => contract::query::owner(deps, STATE),
        QueryMsg::Winner {} => contract::query::winner(deps, env, STATE),
        QueryMsg::QueryBettor { bettor } => {
            contract::query::bettor_count(deps, env, bettor, BETTORS)
        }
    }
}
