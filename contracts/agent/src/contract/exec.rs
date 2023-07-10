use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, Storage};
use cw_storage_plus::{Item, Map};
use cw_utils::must_pay;

use crate::{
    state::{BetInfo, State},
    ContractError, LOTTERY_FEE,
};

#[allow(clippy::too_many_arguments)]
pub fn buy(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    addr: String,
    memo: Option<String>,
    denom: String,
    state: Item<State>,
    joins: Map<&Addr, BetInfo>,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}
