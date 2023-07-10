#[cfg(test)]
mod tests;

use cosmwasm_std::{coins, from_binary, Addr, Coin};
use cw_multi_test::{App, AppResponse, ContractWrapper, Executor};

use anyhow::Result as AnyResult;
use cw_utils::parse_execute_response_data;
use std::convert::Into;

use crate::{msg::*, *};

#[derive(Clone, Debug, Copy)]
pub struct OperatorCodeId(u64);

impl OperatorCodeId {
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

impl From<OperatorCodeId> for u64 {
    fn from(code_id: OperatorCodeId) -> Self {
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

    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: OperatorCodeId,
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
    #[track_caller]
    pub fn create_lottery(
        &self,
        app: &mut App,
        sender: Addr,
        lottery_code_id: u64,
        title: &str,
    ) -> AnyResult<Option<InstantiationData>> {
        let msg = ExecuteMsg::CreateLottery {
            lottery_code_id,
            title: title.into(),
        };

        let resp = self.execute_contract(app, sender, msg, &[])?;

        // resp.data
        //     .map(|d| parse_execute_response_data(&d))
        //     .transpose()?
        //     .and_then(|d| d.data)
        //     .map(|d| from_binary(&d))
        //     .transpose()
        //     .map_err(Into::into)
        Ok(None)
    }

    #[track_caller]
    pub fn close_lottery(
        &self,
        app: &mut App,
        sender: Addr,
        lottery: &str,
        rewards: Vec<Coin>,
    ) -> AnyResult<AppResponse> {
        let msg = ExecuteMsg::CloseLottery {
            lottery: lottery.into(),
            rewards,
        };
        self.execute_contract(app, sender, msg, &[])
    }

    #[track_caller]
    pub fn draw_lottery(
        &self,
        app: &mut App,
        sender: Addr,
        lottery: &str,
    ) -> AnyResult<AppResponse> {
        let msg = ExecuteMsg::DrawLottery {
            lottery: lottery.into(),
        };
        self.execute_contract(app, sender, msg, &[])
    }

    pub fn latest_lottery(&self, app: &App) -> StdResult<LatestLotteryResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::LatestLottery {})
    }

    pub fn execute_contract(
        &self,
        app: &mut App,
        sender: Addr,
        msg: ExecuteMsg,
        send_funds: &[Coin],
    ) -> AnyResult<AppResponse> {
        app.execute_contract(sender, self.addr(), &msg, send_funds)
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
