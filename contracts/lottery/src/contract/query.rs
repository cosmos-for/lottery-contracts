use cosmwasm_std::{to_binary, Addr, Binary, Deps, Env, StdResult};
use cw_storage_plus::{Item, Map};

use crate::{
    msg::{CurrentStateResp, OwnerResp, QueryBettorResp, WinnerResp},
    state::{BetInfo, State},
};

pub fn owner(deps: Deps, state: Item<State>) -> StdResult<Binary> {
    let state = state.load(deps.storage)?;
    to_binary(&OwnerResp { owner: state.owner })
}
pub fn winner(deps: Deps, _env: Env, state: Item<State>) -> StdResult<Binary> {
    let state = state.may_load(deps.storage)?;
    to_binary(&WinnerResp {
        winner: state.and_then(|s| s.winner),
    })
}

pub fn bettor_count(
    deps: Deps,
    _env: Env,
    addr: String,
    bettors: Map<&Addr, BetInfo>,
) -> StdResult<Binary> {
    let info = bettors.may_load(deps.storage, &deps.api.addr_validate(&addr)?)?;

    to_binary(&QueryBettorResp { info })
}

pub fn current_state(deps: Deps, state_item: Item<State>) -> StdResult<Binary> {
    let state = state_item.load(deps.storage)?;
    to_binary(&CurrentStateResp { state })
}
