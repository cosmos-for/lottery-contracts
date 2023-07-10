#[cfg(test)]
mod tests;

use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{App, AppResponse, ContractWrapper, Executor};

use anyhow::Result as AnyResult;
use std::convert::Into;

use crate::{msg::*, *};

#[derive(Clone, Debug, Copy)]
pub struct MembershipCodeId(u64);

impl MembershipCodeId {
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
    ) -> AnyResult<MembershipContract> {
        MembershipContract::instantiate(app, self, sender, title, label)
    }
}

impl From<MembershipCodeId> for u64 {
    fn from(code_id: MembershipCodeId) -> Self {
        code_id.0
    }
}

#[derive(Debug, Clone)]
pub struct MembershipContract(Addr);

// implement the contract real function, e.g. instantiate, functions in exec, query modules
impl MembershipContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: MembershipCodeId,
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

    // 解释创建lottery的结果 TODO
    // #[track_caller]
    // pub fn create_lottery(
    //     &self,
    //     app: &mut App,
    //     sender: Addr,
    //     lottery_code_id: u64,
    //     title: &str,
    // ) -> AnyResult<Option<InstantiationData>> {
    //     // let msg = ExecuteMsg::CreateLottery {
    //     //     lottery_code_id,
    //     //     title: title.into(),
    //     // };

    //     // self.execute_contract(app, sender, msg, &[])?;

    //     Ok(None)
    // }

    // pub fn latest_lottery(&self, app: &App) -> StdResult<LatestLotteryResp> {
    //     app.wrap()
    //         .query_wasm_smart(self.addr(), &QueryMsg::LatestLottery {})
    // }

    pub fn execute_contract(
        &self,
        app: &mut App,
        sender: Addr,
        msg: ExecuteMsg,
        send_funds: &[Coin],
    ) -> AnyResult<AppResponse> {
        app.execute_contract(sender, self.addr(), &msg, send_funds)
    }

}

impl From<Addr> for MembershipContract {
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
