pub mod exec;
pub mod query;
pub mod reply;

use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;
use cw_storage_plus::Item;

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{Config, CONFIG, LATEST_LOTTERY, LOTTERIES},
    ContractError,
};

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const INITIAL_LOTTERY_INSTANTIATION_REPLY_ID: u64 = 1;

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
    state_item: Item<Config>,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        title: msg.title,
        owner: info.sender,
        counter: 0,
    };

    state_item.save(deps.storage, &config)?;

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
        CreateLottery {
            lottery_code_id,
            title,
        } => exec::create_lottery(deps, env, info, lottery_code_id, title, CONFIG),
        CloseLottery { lottery } => exec::close_lottery(deps, env, info, lottery),
        DrawLottery { lottery } => exec::draw_lottery(deps, env, info, lottery),
    }
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        LatestLottery {} => {
            query::latest_lottery(deps, LATEST_LOTTERY).and_then(|resp| to_binary(&resp))
        }
        LotteriesCount {} => query::lotteries_count(deps, CONFIG).and_then(|resp| to_binary(&resp)),
    }
}

pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        INITIAL_LOTTERY_INSTANTIATION_REPLY_ID => {
            reply::initial_lottery_instantiate(deps, env, reply.result.into_result(), LOTTERIES)
        }
        id => Err(ContractError::UnRecognizedReplyIdErr { id }),
    }
}
