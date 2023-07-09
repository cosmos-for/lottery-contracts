use cosmwasm_std::coins;
use cw_multi_test::App;
use lottery::multitest::{LotteryCodeId, LotteryContract};

use crate::{ContractError, NATIVE_DENOM};

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
}

#[test]
fn create_lottery_should_works() {
    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &alice(), coins(1000, NATIVE_DENOM))
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

    assert!(resp.is_some());

    let lottery_addr = resp.unwrap().addr;
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

    contract
        .close_lottery(&mut app, owner(), lottery_addr.as_str())
        .unwrap();

    let winner = lottery.winner(&app).unwrap();

    assert_eq!(winner.winner, Some(alice()))
}
