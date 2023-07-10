pub mod exec;
pub mod query;
pub mod reply;

use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Reply, Response};
use cw2::set_contract_version;
use cw_storage_plus::Item;

use crate::{state::State, ContractError};

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
        owner: Addr::unchecked(owner),
    };

    state_item.save(deps.storage, &state)?;

    Ok(Response::new())
}

pub fn reply(_deps: DepsMut, _env: Env, _reply: Reply) -> Result<Response, ContractError> {
    todo!()
}
