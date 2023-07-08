use cosmwasm_std::{to_binary, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg};
use cw_storage_plus::Item;

use crate::{state::Config, ContractError};

use lottery::msg::InstantiateMsg as LotterInstantiateMsg;

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
