use cosmwasm_std::coins;
use cw_multi_test::App;

use crate::{ContractError, NATIVE_DENOM};

use super::{alice, owner, CodeId};

#[test]
fn instantiate_should_works() {
    let mut app = App::default();
    let code_id = CodeId::store_code(&mut app);
    let title = "operator title";
    let contract = code_id
        .instantiate(&mut app, owner(), title, "operator test")
        .unwrap();

    let lottery = contract.latest_lottery(&app).unwrap();
    assert!(lottery.lottery.is_none());

    let lotteries_count = contract.lotteries_count(&app).unwrap();
    assert_eq!(lotteries_count.counter, 0);
}
