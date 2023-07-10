use cosmwasm_std::{to_binary, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg};
use cw_storage_plus::Item;

use crate::{state::Config, ContractError};

use agent::msg::InstantiateMsg as AgentInstantiateMsg;

use super::CREATED_AGENT_REPLY_ID;

// Create lottery contract, and return the lottery address
pub fn create_agent(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
    config: Item<Config>,
) -> Result<Response, ContractError> {
    let config = config.load(deps.storage)?;
    let sender = &info.sender;

    let init_msg = AgentInstantiateMsg {
        name: name.clone(),
        owner: sender.to_string(),
    };

    let msg = WasmMsg::Instantiate {
        admin: Some(sender.to_string()),
        code_id: config.agent_code_id,
        msg: to_binary(&init_msg)?,
        funds: vec![],
        label: format!("{} agent", name),
    };

    let msg = SubMsg::reply_on_success(msg, CREATED_AGENT_REPLY_ID);

    let resp = Response::new().add_submessage(msg);

    Ok(resp)
}
