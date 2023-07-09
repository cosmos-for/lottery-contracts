use cosmwasm_std::{to_binary, Addr, DepsMut, Env, Response, StdError, SubMsgResponse};
use cw_storage_plus::Map;
use cw_utils::parse_instantiate_response_data;

use crate::{msg::InstantiationData, ContractError};

pub fn initial_lottery_instantiate(
    deps: DepsMut,
    env: Env,
    reply: Result<SubMsgResponse, String>,
    lotteries: Map<&Addr, u64>,
) -> Result<Response, ContractError> {
    let response = reply.map_err(StdError::generic_err)?;
    let data = response.data.ok_or(ContractError::DataMissingErr {})?;
    let response = parse_instantiate_response_data(&data)?;

    let addr = Addr::unchecked(response.contract_address);

    lotteries.save(deps.storage, &addr, &env.block.height)?;

    let data = InstantiationData { addr: addr.clone() };
    let resp = Response::new()
        .add_attribute("action", "initial_lottery_instantiate")
        .add_attribute("lottery_addr", addr.to_string())
        .set_data(to_binary(&data)?);

    Ok(resp)
}
