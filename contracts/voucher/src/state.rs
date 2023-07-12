use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub admin: Addr,
}

#[cw_serde]
pub struct Trait {
    pub trait_type: String,
    pub value: String,
}

#[cw_serde]
pub struct TokenInfo {
    pub contract_address: Addr,
    pub token_type: Trait,
}

// admin must specify the token including contract_address and type of nft
#[cw_serde]
pub struct AllowedTokens {
    pub tokens: Vec<TokenInfo>,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const ALLOWED_TOKENS: Item<AllowedTokens> = Item::new("allowed_tokens");

// This map will store the amount of voucher tokens that user can take
// Each time user burn a voucher token, the amount will be decreased by 1
// Mapping: (user_address, contract_info) => amount
pub const WHITELIST: Map<(Addr, Addr, String), u64> = Map::new("whitelist");
