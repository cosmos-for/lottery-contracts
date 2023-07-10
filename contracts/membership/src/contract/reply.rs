use cosmwasm_std::{to_binary, Addr, DepsMut, Env, Response, StdError, SubMsgResponse};
use cw_storage_plus::{Item, Map};
use cw_utils::parse_instantiate_response_data;

// use crate::{msg::InstantiationData, state::Config, ContractError};

// pub fn initial_lottery_instantiated(
//     deps: DepsMut,
//     env: Env,
//     reply: Result<SubMsgResponse, String>,
//     lotteries: Map<&Addr, u64>,
//     config: Item<Config>,
//     latest_lottery: Item<Addr>,
// ) -> Result<Response, ContractError> {
//     let response = reply.map_err(StdError::generic_err)?;
//     let data = response.data.ok_or(ContractError::DataMissingErr {})?;
//     let response = parse_instantiate_response_data(&data)?;

//     let addr = Addr::unchecked(response.contract_address);

//     lotteries.save(deps.storage, &addr, &env.block.height)?;
//     latest_lottery.save(deps.storage, &addr)?;
//     config.update(deps.storage, |mut config| -> Result<_, ContractError> {
//         config.counter += 1;
//         Ok(config)
//     })?;

//     let data = InstantiationData { addr: addr.clone() };
//     let resp = Response::new()
//         .add_attribute("action", "initial_lottery_instantiate")
//         .add_attribute("lottery_addr", addr.to_string())
//         .set_data(to_binary(&data)?);

//     Ok(resp)
// }

// pub fn closed_lottery(
//     _deps: DepsMut,
//     _env: Env,
//     _reply: Result<SubMsgResponse, String>,
// ) -> Result<Response, ContractError> {
//     // let response = reply.map_err(StdError::generic_err)?;
//     // let data = response.data.ok_or(ContractError::DataMissingErr {})?;

//     // let _response = parse_execute_response_data(&data)?;
//     // let data = response.data.map(|d| from_binary(&d)?);

//     Ok(Response::new())
// }
