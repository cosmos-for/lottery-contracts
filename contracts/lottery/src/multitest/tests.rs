#[cfg(test)]
mod test {
    use cosmwasm_std::coins;
    use cw_multi_test::App;

    use crate::{
        multitest::{alice, owner, LotteryCodeId},
        state::BetInfo,
        ContractError, NATIVE_DENOM,
    };

    #[test]
    fn instantiate_should_works() {
        let mut app = App::default();
        let code_id = LotteryCodeId::store_code(&mut app);
        let title = "lottery title";
        let contract = code_id
            .instantiate(&mut app, owner(), title, "lottery test")
            .unwrap();

        let winner = contract.winner(&app).unwrap();
        assert_eq!(winner.winner, None);

        let contract_owner = contract.owner(&app).unwrap();
        assert_eq!(contract_owner.owner, owner());
    }

    #[test]
    fn buy_lottery_should_works() {
        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &alice(), coins(3000, NATIVE_DENOM))
                .unwrap();
        });

        let code_id = LotteryCodeId::store_code(&mut app);
        let title = "lottery title";
        let contract = code_id
            .instantiate(&mut app, owner(), title, "lottery test")
            .unwrap();

        contract
            .buy(
                &mut app,
                alice(),
                NATIVE_DENOM,
                Some("恭喜发财!".to_string()),
                &coins(100, NATIVE_DENOM),
            )
            .unwrap();
        let resp = contract.bettor_count(&app, alice().as_str()).unwrap();
        let expected = BetInfo {
            buy_at: 12345,
            memo: Some("恭喜发财!".to_string()),
        };
        assert_eq!(resp.info, Some(expected));
    }

    #[test]
    fn close_lottery_should_fail() {
        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &alice(), coins(3000, NATIVE_DENOM))
                .unwrap();
        });

        let code_id = LotteryCodeId::store_code(&mut app);
        let title = "lottery title";
        let contract = code_id
            .instantiate(&mut app, owner(), title, "lottery test")
            .unwrap();

        let err = contract
            .close(&mut app, alice(), &coins(1000, NATIVE_DENOM))
            .unwrap_err();
        assert_eq!(ContractError::UnauthorizedErr {}, err.downcast().unwrap())
    }
}
