use cosmwasm_std::{DepsMut, Env, Reply, Response};

use crate::ContractError;

pub fn reply(_deps: DepsMut, _env: Env, _reply: Reply) -> Result<Response, ContractError> {
    todo!()
}
