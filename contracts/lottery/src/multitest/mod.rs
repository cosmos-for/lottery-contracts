use cosmwasm_std::Addr;
use cw_multi_test::{App, AppResponse, ContractWrapper, Executor};

use anyhow::Result as AnyResult;
use std::convert::Into;

use crate::{
    msg::{QueryBettorResp, WinnerResp},
    *,
};

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
    ) -> AnyResult<Lottery> {
        Lottery::instantiate(app, self, sender, title, label)
    }
}

impl From<CodeId> for u64 {
    fn from(code_id: CodeId) -> Self {
        code_id.0
    }
}

#[derive(Debug, Clone)]
pub struct Lottery(Addr);

// implement the contract real function, e.g. instantiate, functions in exec, query modules
impl Lottery {
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

    pub fn buy(
        &self,
        app: &mut App,
        sender: Addr,
        denom: &str,
        memo: Option<String>,
    ) -> AnyResult<AppResponse> {
        app.execute_contract(
            sender,
            self.addr(),
            &ExecuteMsg::Buy {
                denom: denom.into(),
                memo,
            },
            &[],
        )
    }

    pub fn close(&self, app: &mut App, sender: Addr) -> AnyResult<AppResponse> {
        app.execute_contract(sender, self.addr(), &ExecuteMsg::Close {}, &[])
    }

    pub fn winner(&self, app: &App) -> StdResult<WinnerResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::Winner {})
    }

    pub fn bettor_count(&self, app: &App, bettor: &str) -> StdResult<QueryBettorResp> {
        app.wrap().query_wasm_smart(
            self.addr(),
            &QueryMsg::QueryBettor {
                bettor: bettor.into(),
            },
        )
    }
}

impl From<Addr> for Lottery {
    fn from(value: Addr) -> Self {
        Self(value)
    }
}
