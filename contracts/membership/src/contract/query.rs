use cosmwasm_std::{Addr, Deps, StdResult};
use cw_storage_plus::Item;

use crate::{
    state::Config,
};

// pub fn latest_lottery(deps: Deps, latest_lottery: Item<Addr>) -> StdResult<LatestLotteryResp> {
//     let lottery = latest_lottery.may_load(deps.storage)?;

//     Ok(LatestLotteryResp { lottery })
// }

// pub fn lotteries_count(deps: Deps, config: Item<Config>) -> StdResult<LotteriesCountResp> {
//     let config = config.load(deps.storage)?;
//     Ok(LotteriesCountResp {
//         counter: config.counter,
//     })
// }
