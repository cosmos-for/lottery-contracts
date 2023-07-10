use cosmwasm_std::StdError;
use cw_utils::ParseReplyError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Un recognized reply id {id}")]
    UnRecognizedReplyIdErr { id: u64 },

    #[error("Data missing")]
    DataMissingErr {},

    #[error("{0}")]
    ParseErr(#[from] ParseReplyError),
}
