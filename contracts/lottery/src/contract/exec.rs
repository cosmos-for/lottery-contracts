use cosmwasm_std::{
    coin, ensure, Addr, BankMsg, Coin, DepsMut, Env, MessageInfo, Response, Storage,
};
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

pub fn draw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    state_item: Item<State>,
    bettors: Map<&Addr, BetInfo>,
) -> Result<Response, ContractError> {
    let sender = info.sender;
    let rewards = info.funds;
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

    let winner = choose_winner(bettors, deps.storage)?;

    ensure!(winner.is_some(), ContractError::LotteryNoBettorErr {});

    // Set the rewards
    state.rewards = rewards;

    state.owner = winner.clone().unwrap();

    // Choose the winner
    state.winner = winner;

    state_item.save(deps.storage, &state)?;

    Ok(Response::new())
}

pub fn withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: u128,
    denom: &str,
    state_item: Item<State>,
    withdraws: Map<&Addr, Vec<Coin>>,
) -> Result<Response, ContractError> {
    let sender = info.sender;
    let state = state_item.load(deps.storage)?;

    if sender != state.owner {
        return Err(ContractError::UnauthorizedErr {});
    }

    let balance = deps.querier.query_balance(env.contract.address, denom)?;
    if balance.amount.u128() < amount {
        return Err(ContractError::WidthrawAmountTooMuchErr {
            amount,
            denom: denom.into(),
        });
    }

    let ws = withdraws.may_load(deps.storage, &sender)?;

    let mut ws = ws.unwrap_or_default();
    let withdraw_coin = coin(amount, denom);
    ws.push(withdraw_coin.clone());

    withdraws.save(deps.storage, &sender, &ws)?;

    let bank_msg = BankMsg::Send {
        to_address: sender.as_str().into(),
        amount: vec![withdraw_coin],
    };

    Ok(Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "withdraw")
        .add_attribute("sender", sender.as_str()))
}

pub fn choose_winner(
    bettors: Map<&Addr, BetInfo>,
    storage: &dyn Storage,
) -> Result<Option<Addr>, ContractError> {
    let winner = bettors.first(storage)?;
    Ok(winner.map(|(k, _)| k))
}
