use cosmwasm_std::{to_binary, Addr, DepsMut, Empty, Env, Response, StdError, SubMsgResponse};
use cw_storage_plus::{Item, Map};
use cw_utils::parse_instantiate_response_data;

use crate::{msg::InstantiationData, state::Config, ContractError};

pub fn created_agent(
    deps: DepsMut,
    _env: Env,
    reply: Result<SubMsgResponse, String>,
    agents_item: Item<Vec<Addr>>,
    config: Item<Config>,
    agent_map: Map<&Addr, Empty>,
) -> Result<Response, ContractError> {
    let response = reply.map_err(StdError::generic_err)?;
    let data = response.data.ok_or(ContractError::DataMissingErr {})?;
    let response = parse_instantiate_response_data(&data)?;

    let addr = &Addr::unchecked(response.contract_address);

    let agents = agents_item.may_load(deps.storage)?;
    let mut agents = agents.unwrap_or_default();
    agents.push(addr.clone());
    agents_item.save(deps.storage, &agents)?;

    agent_map.save(deps.storage, addr, &Empty {})?;
    config.update(deps.storage, |mut config| -> Result<_, ContractError> {
        config.counter += 1;
        Ok(config)
    })?;

    let data = InstantiationData { addr: addr.clone() };
    let resp = Response::new()
        .add_attribute("action", "created_agent")
        .add_attribute("agent_addr", addr.to_string())
        .set_data(to_binary(&data)?);

    Ok(resp)
}
