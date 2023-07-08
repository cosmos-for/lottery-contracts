use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    StdErr(#[from] StdError),

    #[error("Unauthorized")]
    UnauthorizedErr {},

    #[error("Agent {agent} already bought {addr}")]
    CantBuyLastLotteryErr { addr: String, agent: String },

    #[error("{0}")]
    PaymentErr(#[from] PaymentError),

    #[error("Payment not enough")]
    PaymentNotEnoughErr {},

    #[error("Current block height: {height} is less than lottery sequence: {sequence}")]
    LotterySequenceNotMatchErr { height: u64, sequence: u64 },

    #[error("Lottery: {addr} is already closed")]
    LotteryIsAlreadyClosedErr { addr: String },
}
