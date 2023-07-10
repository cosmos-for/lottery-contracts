use cosmwasm_std::{coins, to_binary, Addr, DepsMut, Env, MessageInfo, Response, WasmMsg};
use cw_storage_plus::{Item, Map};
use cw_utils::must_pay;

use crate::{
    state::{BetInfo, State},
    ContractError,
};

#[allow(clippy::too_many_arguments)]
pub fn buy_lottery(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    addr: String,
    memo: Option<String>,
    denom: String,
    state_item: Item<State>,
    joins: Item<Vec<Addr>>,
    join_map: Map<&Addr, BetInfo>,
) -> Result<Response, ContractError> {
    let sender = &info.sender;
    let amount = must_pay(&info, &denom)?.u128();
    let mut state = state_item.load(deps.storage)?;

    if sender != state.owner {
        return Err(ContractError::UnauthorizedErr {});
    }

    let msg = lottery::msg::ExecuteMsg::Buy {
        denom: denom.clone(),
        memo: memo.clone(),
    };

    state.counter += 1;
    state_item.save(deps.storage, &state)?;

    let joined = joins.may_load(deps.storage)?;
    let mut joined = joined.unwrap_or_default();
    joined.push(Addr::unchecked(addr.clone()));

    joins.save(deps.storage, &joined)?;

    join_map.save(
        deps.storage,
        &Addr::unchecked(addr.clone()),
        &BetInfo {
            buy_at: env.block.height,
            memo,
        },
    )?;

    let msg = WasmMsg::Execute {
        contract_addr: addr,
        msg: to_binary(&msg)?,
        funds: coins(amount, denom),
    };

    let resp = Response::new()
        .add_message(msg)
        .add_attribute("action", "buy_lottery")
        .add_attribute("sender", sender.as_str());

    Ok(resp)
}
