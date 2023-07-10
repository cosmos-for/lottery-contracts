pub mod exec;
pub mod query;
pub mod reply;

use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use cw2::set_contract_version;
use cw_storage_plus::{Item, Map};

use crate::{
    contract::exec::buy_lottery,
    msg::{ExecuteMsg, QueryMsg},
    state::{BetInfo, State, LOTTERIES_JOINED, STATE},
    ContractError,
};

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    name: String,
    owner: String,
    state_item: Item<State>,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let height = env.block.height;
    let created_at = env.block.time.seconds();

    let state = State {
        name,
        height,
        created_at,
        counter: 0,
        owner: Addr::unchecked(owner),
    };

    state_item.save(deps.storage, &state)?;

    Ok(Response::new())
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
    state_item: Item<State>,
    joins: Item<Vec<Addr>>,
    join_map: Map<&Addr, BetInfo>,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;
    match msg {
        Buy { addr, denom, memo } => buy_lottery(
            deps, env, info, addr, memo, denom, state_item, joins, join_map,
        ),
    }
}
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        LotteriesJoined {} => query::lotteries_joined(deps, LOTTERIES_JOINED),
        LotteriesWinned {} => todo!(),
        CurrentState {} => query::current_state(deps, STATE),
    }
}
pub fn reply(_deps: DepsMut, _env: Env, _reply: Reply) -> Result<Response, ContractError> {
    todo!()
}
