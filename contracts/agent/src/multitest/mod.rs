mod tests;

use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{App, AppResponse, ContractWrapper, Executor};

use anyhow::Result as AnyResult;
use std::convert::Into;

use crate::{
    msg::{CurrentStateResp, LotteriesWinnedResp},
    *,
};

#[derive(Clone, Debug, Copy)]
pub struct AgentCodeId(u64);

impl AgentCodeId {
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
        owner: &str,
        label: &str,
    ) -> AnyResult<AgentContract> {
        AgentContract::instantiate(app, self, sender, title, owner, label)
    }
}

impl From<AgentCodeId> for u64 {
    fn from(code_id: AgentCodeId) -> Self {
        code_id.0
    }
}

#[derive(Debug, Clone)]
pub struct AgentContract(Addr);

// implement the contract real function, e.g. instantiate, functions in exec, query modules
impl AgentContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn from_addr(addr: Addr) -> Self {
        Self(addr)
    }

    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: AgentCodeId,
        sender: Addr,
        name: &str,
        owner: &str,
        label: &str,
    ) -> AnyResult<Self> {
        app.instantiate_contract(
            code_id.0,
            Addr::unchecked(sender),
            &InstantiateMsg {
                name: name.into(),
                owner: owner.into(),
            },
            &[],
            label,
            None,
        )
        .map(Self::from)
    }

    #[track_caller]
    pub fn buy_lottery(
        &self,
        app: &mut App,
        sender: Addr,
        addr: &str,
        denom: &str,
        memo: Option<String>,
        funds: &[Coin],
    ) -> AnyResult<AppResponse> {
        app.execute_contract(
            sender,
            self.addr(),
            &ExecuteMsg::Buy {
                addr: addr.into(),
                denom: denom.into(),
                memo,
            },
            funds,
        )
    }

    pub fn lotteries_joined(&self, app: &App) -> StdResult<LotteriesJoinedResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::LotteriesJoined {})
    }

    pub fn query_balances(app: &App, addr: Addr) -> StdResult<Vec<Coin>> {
        app.wrap().query_all_balances(addr)
    }

    pub fn current_state(&self, app: &App) -> StdResult<CurrentStateResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::CurrentState {})
    }
}

impl From<Addr> for AgentContract {
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
