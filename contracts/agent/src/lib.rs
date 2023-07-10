pub mod contract;
mod error;
pub mod msg;
pub mod state;

#[cfg(any(feature = "mt", test))]
pub mod multitest;

pub use crate::error::ContractError;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use msg::LotteriesJoinedResp;
use state::STATE;

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
    contract::instantiate(deps, env, info, msg.name, msg.owner, STATE)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;
    match msg {
        Buy { addr, denom, memo } => todo!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    contract::reply(deps, env, reply)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        LotteriesJoined {} => todo!(),
        LotteriesWinned {} => todo!(),
        CurrentState {} => todo!(),
    }
}
