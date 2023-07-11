use cosmwasm_std::{to_binary, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg, coins, Coin, Addr};
use cw_storage_plus::{Item, Map};

use crate::{state::Config, ContractError, NATIVE_DENOM};

use lottery::msg::{ExecuteMsg as LotteryExecuteMsg, InstantiateMsg as LotterInstantiateMsg};

use super::INITIAL_LOTTERY_INSTANTIATION_REPLY_ID;

// Create lottery contract, and return the lottery address
pub fn create_lottery(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    lottery_code_id: u64,
    title: String,
    config: Item<Config>,
) -> Result<Response, ContractError> {
    let config = config.load(deps.storage)?;
    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let init_msg = LotterInstantiateMsg {
        title: title.clone(),
    };

    let msg = WasmMsg::Instantiate {
        admin: Some(env.contract.address.to_string()),
        code_id: lottery_code_id,
        msg: to_binary(&init_msg)?,
        funds: vec![],
        label: format!("{} lottery", title),
    };

    let msg = SubMsg::reply_on_success(msg, INITIAL_LOTTERY_INSTANTIATION_REPLY_ID);

    let resp = Response::new().add_submessage(msg);

    Ok(resp)
}

pub fn draw_lottery(
    deps: DepsMut,
    info: MessageInfo,
    lottery: String,
    config: Item<Config>,
    lottery_rewards: Map<&Addr, Vec<Coin>>,
) -> Result<Response, ContractError> {
    let sender = info.sender;
    let config = config.load(deps.storage)?;
    if config.owner != sender {
        return Err(ContractError::Unauthorized {});
    }

    let funds = calculate_lottery_rewards(lottery.as_str());
    lottery_rewards.save(deps.storage, &Addr::unchecked(&lottery), &funds)?;

    let close_msg = LotteryExecuteMsg::Draw {};
    let msg = WasmMsg::Execute {
        contract_addr: lottery.clone(),
        msg: to_binary(&close_msg)?,
        funds,
    };

    let resp = Response::new()
        .add_message(msg)
        .add_attribute("action", "close_lottery")
        .add_attribute("sender", sender)
        .add_attribute("lottery", lottery);

    Ok(resp)
}

pub fn calculate_lottery_rewards(_lottery: &str) -> Vec<Coin> {
    coins(1000, NATIVE_DENOM)
}