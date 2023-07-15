use std::ops::Rem;

use cosmwasm_std::{coins, Uint128};
use cw_multi_test::App;
use lottery::multitest::{LotteryCodeId, LotteryContract};

use crate::NATIVE_DENOM;

use super::{alice, owner, OperatorCodeId};

#[test]
fn instantiate_should_works() {
    let mut app = App::default();
    let code_id = OperatorCodeId::store_code(&mut app);
    let title = "operator title";
    let contract = code_id
        .instantiate(&mut app, owner(), title, "operator test")
        .unwrap();

    let lottery = contract.latest_lottery(&app).unwrap();
    assert!(lottery.lottery.is_none());

    let lotteries_count = contract.lotteries_count(&app).unwrap();
    assert_eq!(lotteries_count.counter, 0);

    let t = Uint128::new(100);
    let r = t.rem(Uint128::new(1000));
}

#[test]
fn create_lottery_should_works() {
    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &alice(), coins(1000, NATIVE_DENOM))
            .unwrap();
        router
            .bank
            .init_balance(storage, &owner(), coins(2000, NATIVE_DENOM))
            .unwrap();
    });
    let code_id = OperatorCodeId::store_code(&mut app);
    let lottery_code_id = LotteryCodeId::store_code(&mut app);
    let title = "operator title";
    let contract = code_id
        .instantiate(&mut app, owner(), title, "operator test")
        .unwrap();

    let resp = contract
        .create_lottery(&mut app, owner(), lottery_code_id.into(), "create lottery")
        .unwrap();
    assert!(resp.is_none());

    let lotteries_count = contract.lotteries_count(&app).unwrap();
    assert_eq!(lotteries_count.counter, 1);

    let latest_lottery = contract.latest_lottery(&app).unwrap();
    assert!(latest_lottery.lottery.is_some());

    // assert!(resp.is_some());

    let lottery_addr = latest_lottery.lottery.unwrap();
    let lottery = LotteryContract::from_addr(lottery_addr.clone());

    lottery
        .buy(
            &mut app,
            alice(),
            NATIVE_DENOM,
            Some("alice buy lottery".into()),
            &coins(100, NATIVE_DENOM),
        )
        .unwrap();

    // lottery
    //     .close(&mut app, contract.addr(), coins(1000, NATIVE_DENOM))
    //     .unwrap();

    contract
        .close_lottery(
            &mut app,
            owner(),
            lottery_addr.as_str(),
            &coins(1000, NATIVE_DENOM),
        )
        .unwrap();

    let winner = lottery.winner(&app).unwrap();

    assert_eq!(winner.winner, Some(alice()));

    let lottery_balances = LotteryContract::query_balances(&app, lottery_addr).unwrap();

    assert_eq!(lottery_balances, coins(1100, NATIVE_DENOM));

    let rewards = lottery.query_state(&app).unwrap();
    assert_eq!(rewards.state.rewards, coins(1000, NATIVE_DENOM));
}
