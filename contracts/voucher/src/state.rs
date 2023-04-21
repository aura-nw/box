use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub admin: Addr,
}

#[cw_serde]
pub struct AllowedContracts {
    pub contract_address: Vec<Addr>,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const ALLOWED_CONTRACTS: Item<AllowedContracts> = Item::new("allowed_contracts");

// This map will store the amount of voucher tokens that user can take
// Each time user burn a voucher token, the amount will be decreased by 1
// Mapping: (user_address, nft_contract_address) => amount
pub const WHITELIST: Map<(Addr, Addr), u64> = Map::new("whitelist");
