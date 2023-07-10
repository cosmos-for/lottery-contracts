use cosmwasm_std::{Addr, Coin, DepsMut, Env, MessageInfo, Response, Storage};
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
    // addr: String,
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
    let lottery_sequnce = state.height;

    let contract_addr = env.contract.address;
    let block_height = env.block.height;

    // Only can buy lottery after created block height
    if state.height > block_height {
        return Err(ContractError::LotterySequenceNotMatchErr {
            height: block_height,
            sequence: lottery_sequnce,
        });
    }
    // Can't buy lottery after lottery is already closed
    if state.winner.is_some() {
        return Err(ContractError::LotteryIsAlreadyClosedErr {
            addr: contract_addr,
        });
    }

    let sender = info.sender;
    let bettor = bettors.may_load(deps.storage, &sender)?;

    // Only can buy lottery once
    match bettor {
        Some(_) => Err(ContractError::OnlyBuyLotteryOnceErr {
            agent: sender.to_string(),
            addr: contract_addr,
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
    rewards: Vec<Coin>,
    state_item: Item<State>,
    bettors: Map<&Addr, BetInfo>,
) -> Result<Response, ContractError> {
    let sender = info.sender;
    let mut state = state_item.load(deps.storage)?;

    if state.owner != sender {
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
        return Err(ContractError::LotteryIsAlreadyClosedErr {
            addr: env.contract.address,
        });
    }

    // Calculate the rewards TODO
    state.rewards = rewards;

    // Choose the winner TODO
    state.winner = choose_winner(bettors, deps.storage)?;

    state_item.save(deps.storage, &state)?;

    Ok(Response::new())
}

pub fn choose_winner(
    bettors: Map<&Addr, BetInfo>,
    storage: &dyn Storage,
) -> Result<Option<Addr>, ContractError> {
    let winner = bettors.first(storage)?;
    Ok(winner.map(|(k, _)| k))
}
