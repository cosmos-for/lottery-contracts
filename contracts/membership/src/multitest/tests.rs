use agent::multitest::AgentCodeId;
use cosmwasm_std::coins;
use cw_multi_test::App;

use crate::NATIVE_DENOM;

use super::{alice, owner, MembershipCodeId};

#[test]
fn instantiate_should_works() {
    let mut app = App::default();
    let code_id = MembershipCodeId::store_code(&mut app);
    let title = "membership title";
    let agent_code_id = agent::multitest::AgentCodeId::store_code(&mut app);
    let contract = code_id
        .instantiate(
            &mut app,
            owner(),
            title,
            agent_code_id.into(),
            "membership test",
        )
        .unwrap();

    let membership = contract.current_config(&app).unwrap();
    assert_eq!(membership.config.title, title);
    assert_eq!(membership.config.agent_code_id, agent_code_id.id());
    assert_eq!(membership.config.counter, 0);
    assert_eq!(membership.config.owner, owner());
}

#[test]
fn create_agent_should_works() {
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
    let code_id = MembershipCodeId::store_code(&mut app);
    let agent_code_id = AgentCodeId::store_code(&mut app);
    let title = "membership title";
    let contract = code_id
        .instantiate(
            &mut app,
            owner(),
            title,
            agent_code_id.id(),
            "membership test",
        )
        .unwrap();

    let resp = contract
        .create_agent(&mut app, owner(), "create lottery")
        .unwrap();
    assert!(resp.is_none());

    let resp = contract.current_config(&app).unwrap();
    assert_eq!(resp.config.counter, 1);

    let resp = contract.agent_lists(&app).unwrap();
    assert_eq!(resp.agents.len(), 1);
}
