use cosmwasm_std::{to_binary, Addr, Binary, Deps, StdResult};
use cw_storage_plus::Item;

use crate::{
    msg::{CurrentStateResp, LotteriesJoinedResp},
    state::State,
};

pub fn lotteries_joined(deps: Deps, lotteries_joined: Item<Vec<Addr>>) -> StdResult<Binary> {
    let lotteries = lotteries_joined.may_load(deps.storage)?;
    to_binary(&LotteriesJoinedResp {
        lotteries: lotteries.unwrap_or_default(),
    })
}

pub fn current_state(deps: Deps, state_item: Item<State>) -> StdResult<Binary> {
    let state = state_item.load(deps.storage)?;
    to_binary(&CurrentStateResp { state })
}
