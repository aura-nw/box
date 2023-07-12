use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;

use crate::state::{AllowedTokens, Config, Trait};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    AllowToken {
        contract_address: String,
        token_type: Option<Trait>,
    },
    ReceiveNft(Cw721ReceiveMsg),
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
    #[returns(u64)]
    RemainingVouchers {
        owner: String,
        contract_address: String,
        token_type: Option<Trait>,
    },
    #[returns(AllowedTokens)]
    AllowedTokens {},
}

#[cw_serde]
pub struct Cw721ReceiveMsg {
    pub sender: String,
    pub token_id: String,
    pub msg: Binary,
}

#[cw_serde]
pub enum Cw721HookMsg {
    Burn {
        contract_address: String,
        token_id: String,
        token_type: Option<Trait>,
    },
}

// This is just a helper to properly serialize the above message
#[cw_serde]
enum ReceiverExecuteMsg {
    Receive(Cw721ReceiveMsg),
}
