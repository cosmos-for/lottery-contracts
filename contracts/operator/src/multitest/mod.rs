mod tests;

use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{App, AppResponse, ContractWrapper, Executor};

use anyhow::Result as AnyResult;
use std::convert::Into;

use crate::{msg::*, *};

#[derive(Clone, Debug, Copy)]
pub struct CodeId(u64);

impl CodeId {
    pub fn store_code(app: &mut App) -> Self {
        let contract = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
        let code_id = app.store_code(Box::new(contract));
        Self(code_id)
    }

    pub fn instantiate(
        self,
        app: &mut App,
        sender: Addr,
        title: &str,
        label: &str,
    ) -> AnyResult<OperatorContract> {
        OperatorContract::instantiate(app, self, sender, title, label)
    }
}

impl From<CodeId> for u64 {
    fn from(code_id: CodeId) -> Self {
        code_id.0
    }
}

#[derive(Debug, Clone)]
pub struct OperatorContract(Addr);

// implement the contract real function, e.g. instantiate, functions in exec, query modules
impl OperatorContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn instantiate(
        app: &mut App,
        code_id: CodeId,
        sender: Addr,
        title: &str,
        label: &str,
    ) -> AnyResult<Self> {
        app.instantiate_contract(
            code_id.0,
            Addr::unchecked(sender),
            &InstantiateMsg {
                title: title.into(),
            },
            &[],
            label,
            None,
        )
        .map(Self::from)
    }

    // pub fn buy(
    //     &self,
    //     app: &mut App,
    //     sender: Addr,
    //     denom: &str,
    //     memo: Option<String>,
    //     funds: &[Coin],
    // ) -> AnyResult<AppResponse> {
    //     app.execute_contract(
    //         sender,
    //         self.addr(),
    //         &ExecuteMsg::Buy {
    //             denom: denom.into(),
    //             memo,
    //         },
    //         funds,
    //     )
    // }

    // pub fn close(&self, app: &mut App, sender: Addr) -> AnyResult<AppResponse> {
    //     app.execute_contract(sender, self.addr(), &ExecuteMsg::Close {}, &[])
    // }

    pub fn latest_lottery(&self, app: &App) -> StdResult<LatestLotteryResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::LatestLottery {})
    }

    pub fn lotteries_count(&self, app: &App) -> StdResult<LotteriesCountResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::LotteriesCount {})
    }
}

impl From<Addr> for OperatorContract {
    fn from(value: Addr) -> Self {
        Self(value)
    }
}

pub fn alice() -> Addr {
    Addr::unchecked("sei18rszd3tmgpjvjwq2qajtmn5jqvtscd2yuygl4z")
}

pub fn bob() -> Addr {
    Addr::unchecked("sei1aan9kqywf4rf274cal0hj6eyly6wu0uv7edxy2")
}

pub fn owner() -> Addr {
    Addr::unchecked("sei1zj6fjsc2gkce878ukzg6g9wy8cl8p554dlggxd")
}

pub fn parent() -> Addr {
    Addr::unchecked("inj1g9v8suckezwx93zypckd4xg03r26h6ejlmsptz")
}
