#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{Cw721ReceiveMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{AllowedContracts, Config, ALLOWED_CONTRACTS, CONFIG, WHITELIST};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:voucher";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // save contract config
    let config = Config {
        admin: info.sender.clone(),
    };
    CONFIG.save(deps.storage, &config)?;

    // init allowed contract address list
    let allowed_contracts = AllowedContracts {
        contract_address: vec![],
    };
    ALLOWED_CONTRACTS.save(deps.storage, &allowed_contracts)?;

    Ok(
        Response::new()
            .add_attributes([("action", "instantiate"), ("admin", info.sender.as_ref())]),
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    match msg {
        // Find matched incoming message variant and execute them with your custom logic.
        //
        // With `Response` type, it is possible to dispatch message to invoke external logic.
        // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AllowToken { contract_address } => {
            allow_token(deps, env, info, contract_address)
        }
        ExecuteMsg::ReceiveNft(msg) => receive_cw721(deps, env, info, msg),
    }
}

pub fn receive_cw721(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    cw721_msg: Cw721ReceiveMsg,
) -> Result<Response, ContractError> {
    match cw721_msg {
        Cw721ReceiveMsg {
            sender, token_id, ..
        } => {
            // convert contract address to canonical address
            let contract_address = info.sender;

            // if the contract address is not in the allowed list, return error
            let allowed_contracts = ALLOWED_CONTRACTS.load(deps.storage)?;
            if !allowed_contracts
                .contract_address
                .contains(&contract_address)
            {
                return Err(ContractError::NotAllowed {});
            }

            let whitelist_key = (
                deps.api.addr_validate(&sender).unwrap(),
                contract_address.clone(),
            );

            // increase the balance of the voucher for the sender
            // if sender is not in the whitelist, add it to the whitelist
            if WHITELIST
                .may_load(deps.storage, whitelist_key.clone())?
                .is_none()
            {
                WHITELIST.save(deps.storage, whitelist_key, &1)?;
            } else {
                let mut balance = WHITELIST.load(deps.storage, whitelist_key.clone())?;
                balance += 1;
                WHITELIST.save(deps.storage, whitelist_key, &balance)?;
            }

            Ok(Response::new().add_attributes([
                ("action", "receive_cw721"),
                ("sender", sender.as_ref()),
                ("contract_address", contract_address.as_ref()),
                ("token_id", token_id.as_ref()),
            ]))
        }
    }
}

pub fn allow_token(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract_address: String,
) -> Result<Response, ContractError> {
    // if the sender is not admin, return error
    let config = CONFIG.load(deps.storage)?;
    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // convert contract address to canonical address
    let contract_address = deps.api.addr_validate(&contract_address)?;

    // load allowed contract address list
    let allowed_contracts = ALLOWED_CONTRACTS.load(deps.storage)?;

    // if the contract address is already in the list, return error
    if allowed_contracts
        .contract_address
        .contains(&contract_address)
    {
        return Err(ContractError::AlreadyAllowed {});
    }

    // add the contract address to the list
    let mut allowed_contracts = allowed_contracts;
    allowed_contracts
        .contract_address
        .push(contract_address.clone());
    ALLOWED_CONTRACTS.save(deps.storage, &allowed_contracts)?;

    Ok(Response::new().add_attributes([
        ("action", "allow_token"),
        ("contract_address", contract_address.as_ref()),
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::RemainingVouchers {
            owner,
            contract_address,
        } => to_binary(&query_remaining_vouchers(deps, owner, contract_address)?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config)
}

pub fn query_remaining_vouchers(
    deps: Deps,
    owner: String,
    contract_address: String,
) -> StdResult<u64> {
    // convert owner and contract address to canonical address
    let owner = deps.api.addr_validate(&owner)?;
    let contract_address = deps.api.addr_validate(&contract_address)?;

    // load whitelist
    let whitelist_key = (owner, contract_address);

    // if the owner is not in the whitelist, return 0
    if WHITELIST
        .may_load(deps.storage, whitelist_key.clone())?
        .is_none()
    {
        Ok(0)
    } else {
        // if the owner is in the whitelist, return the balance
        let balance = WHITELIST.load(deps.storage, whitelist_key)?;

        Ok(balance)
    }
}
