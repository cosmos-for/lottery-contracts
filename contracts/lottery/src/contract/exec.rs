use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};
use cw_storage_plus::{Item, Map};
use cw_utils::must_pay;

use crate::{
    state::{BetInfo, State},
    ContractError, LOTTERY_FEE,
};

#[allow(clippy::too_many_arguments)]
pub fn buy(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    addr: String,
    memo: Option<String>,
    denom: String,
    state: Item<State>,
    bettors: Map<&Addr, BetInfo>,
) -> Result<Response, ContractError> {
    let amount = must_pay(&info, &denom)?.u128();

    if amount < LOTTERY_FEE {
        return Err(ContractError::PaymentNotEnoughErr {});
    }

    let state = state.load(deps.storage)?;
    let block_height = env.block.height;
    let lottery_sequnce = state.height;
    // Only can buy lottery after created block height
    if state.height > block_height {
        return Err(ContractError::LotterySequenceNotMatchErr {
            height: block_height,
            sequence: lottery_sequnce,
        });
    }
    // Can't buy lottery after lottery is already closed
    if state.winner.is_some() {
        return Err(ContractError::LotteryIsAlreadyClosedErr { addr });
    }

    let sender = info.sender;
    let bettor = bettors.may_load(deps.storage, &sender)?;

    // Only can buy lottery once
    match bettor {
        Some(_) => Err(ContractError::CantBuyLastLotteryErr {
            agent: sender.to_string(),
            addr,
        }),
        None => {
            bettors.save(
                deps.storage,
                &sender,
                &BetInfo {
                    buy_at: block_height,
                    memo,
                },
            )?;
            Ok(Response::new())
        }
    }
}

pub fn close(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    addr: String,
    state: Item<State>,
    _bettors: Map<&Addr, BetInfo>,
) -> Result<Response, ContractError> {
    let sender = info.sender;
    let state = state.load(deps.storage)?;
    let owner = state.owner;
    if owner == sender {
        return Err(ContractError::UnauthorizedErr {});
    }

    let block_height = env.block.height;
    let lottery_sequnce = state.height;
    // Only can buy lottery after created block height
    if state.height > block_height {
        return Err(ContractError::LotterySequenceNotMatchErr {
            height: block_height,
            sequence: lottery_sequnce,
        });
    }
    // Can't buy lottery after lottery is already closed
    if state.winner.is_some() {
        return Err(ContractError::LotteryIsAlreadyClosedErr { addr });
    }

    // Calculate the rewards TODO

    // Choose the winner TODO

    // Set winner of the lottery TOTO

    Ok(Response::new())
}
