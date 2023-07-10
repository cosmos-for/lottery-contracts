use cosmwasm_std::coins;
use cw_multi_test::App;

use crate::{state::BetInfo, ContractError, NATIVE_DENOM};

use super::{alice, owner, AgentCodeId};

#[test]
fn instantiate_should_works() {
    let mut app = App::default();
    let code_id = AgentCodeId::store_code(&mut app);
    let title = "lottery title";
    let agent_owner = owner().to_string();
    let contract = code_id
        .instantiate(&mut app, owner(), title, &agent_owner, "lottery test")
        .unwrap();

    let joined = contract.lotteries_joined(&app).unwrap();
    assert!(joined.lotteries.is_empty());

    let state = contract.current_state(&app).unwrap();
    assert_eq!(state.state.owner, agent_owner,)
}

#[test]
fn buy_lottery_should_works() {
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

    let code_id = AgentCodeId::store_code(&mut app);
    let lottery_id = lottery::multitest::LotteryCodeId::store_code(&mut app);
    let lottery_contract = lottery_id
        .instantiate(&mut app, alice(), "lottery title", "lottery test")
        .unwrap();
    let title = "agent title";
    let agent_owner = owner().to_string();
    let contract = code_id
        .instantiate(&mut app, owner(), title, &agent_owner, "agent test")
        .unwrap();

    contract
        .buy_lottery(
            &mut app,
            owner(),
            lottery_contract.addr().as_str(),
            NATIVE_DENOM,
            Some("恭喜发财!".to_string()),
            &coins(100, NATIVE_DENOM),
        )
        .unwrap();

    let joined = contract.lotteries_joined(&app).unwrap();
    assert_eq!(joined.lotteries.len(), 1);
}
