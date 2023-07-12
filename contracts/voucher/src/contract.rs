#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response,
    StdResult, WasmQuery,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{Cw721HookMsg, Cw721ReceiveMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{AllowedTokens, Config, TokenInfo, Trait, ALLOWED_TOKENS, CONFIG, WHITELIST};

use cw2981_royalties::Metadata;
use cw721::Cw721QueryMsg;

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
    let allowed_tokens = AllowedTokens { tokens: vec![] };
    ALLOWED_TOKENS.save(deps.storage, &allowed_tokens)?;

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
    let api = deps.api;
    match msg {
        ExecuteMsg::AllowToken {
            contract_address,
            token_type,
        } => {
            let token_type = match token_type {
                Some(token_type) => token_type,
                None => Trait {
                    trait_type: "any".to_string(),
                    value: "any".to_string(),
                },
            };
            allow_token(
                deps,
                env,
                info,
                TokenInfo {
                    contract_address: api.addr_validate(&contract_address)?,
                    token_type,
                },
            )
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
    match from_binary(&cw721_msg.msg) {
        Ok(Cw721HookMsg::Burn {
            contract_address,
            token_id,
            token_type,
        }) => {
            // convert contract address to canonical address
            let contract_address = deps.api.addr_validate(&contract_address)?;

            // the infor.sender and contract address must be the same
            if contract_address != info.sender {
                return Err(ContractError::Unauthorized {});
            }

            let token_type = match token_type {
                Some(token_type) => token_type,
                None => Trait {
                    trait_type: "any".to_string(),
                    value: "any".to_string(),
                },
            };

            if !valid_trait(
                deps.as_ref(),
                TokenInfo {
                    contract_address: contract_address.clone(),
                    token_type: token_type.clone(),
                },
                token_id.clone(),
            )? {
                return Err(ContractError::NotAllowed {});
            }

            let whitelist_key = (
                deps.api.addr_validate(&cw721_msg.sender).unwrap(),
                contract_address.clone(),
                format!("{}{}", token_type.trait_type, token_type.value),
            );

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

            return Ok(Response::new().add_attributes([
                ("action", "burn"),
                ("sender", cw721_msg.sender.as_str()),
                ("contract_address", contract_address.as_str()),
                ("token_id", token_id.as_str()),
                ("trait_type", token_type.trait_type.as_str()),
                ("trait_value", token_type.value.as_str()),
            ]));
        }
        Err(_) => Err(ContractError::NotAllowed {}),
    }
}

pub fn allow_token(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    token_info: TokenInfo,
) -> Result<Response, ContractError> {
    // if the sender is not admin, return error
    let config = CONFIG.load(deps.storage)?;
    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // load allowed contract address list
    let allowed_tokens = ALLOWED_TOKENS.load(deps.storage)?;

    // if the contract address is already in the list, return error
    if allowed_tokens.tokens.contains(&token_info) {
        return Err(ContractError::AlreadyAllowed {});
    }

    // add the token info to the list
    let mut allowed_tokens = allowed_tokens;
    allowed_tokens.tokens.push(token_info.clone());
    ALLOWED_TOKENS.save(deps.storage, &allowed_tokens)?;

    Ok(Response::new().add_attributes([
        ("action", "allow_token"),
        ("contract_address", token_info.contract_address.as_ref()),
        ("trait_type", &token_info.token_type.trait_type.to_string()),
        ("trait_value", &token_info.token_type.value.to_string()),
    ]))
}

pub fn valid_trait(
    deps: Deps,
    token_info: TokenInfo,
    token_id: String,
) -> Result<bool, ContractError> {
    let allowed_tokens = ALLOWED_TOKENS.load(deps.storage)?;
    // if the token is not in the allowed list, return error
    if !allowed_tokens.tokens.contains(&token_info) {
        return Ok(false);
    }

    if token_info.token_type.trait_type == "any" && token_info.token_type.value == "any" {
        Ok(true)
    } else {
        // check the type of the nft
        let nft_info_msg = Cw721QueryMsg::NftInfo { token_id };
        let nft_info_response: StdResult<cw721::NftInfoResponse<Metadata>> =
            deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: token_info.contract_address.to_string(),
                msg: to_binary(&nft_info_msg)?,
            }));

        match nft_info_response {
            Ok(nft_info) => {
                if let Some(attributes) = nft_info.extension.attributes {
                    // foreach attribute, check if it is the same as the token type
                    for attribute in attributes {
                        if attribute.trait_type == token_info.token_type.trait_type
                            && attribute.value == token_info.token_type.value
                        {
                            return Ok(true);
                        }
                    }
                    Ok(false)
                } else {
                    Ok(false)
                }
            }
            Err(_) => Ok(false),
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::RemainingVouchers {
            owner,
            contract_address,
            token_type,
        } => to_binary(&query_remaining_vouchers(
            deps,
            owner,
            contract_address,
            token_type,
        )?),
        QueryMsg::AllowedTokens {} => to_binary(&query_allowed_tokens(deps)?),
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
    token_type: Option<Trait>,
) -> StdResult<u64> {
    // convert owner and contract address to canonical address
    let owner = deps.api.addr_validate(&owner)?;
    let contract_address = deps.api.addr_validate(&contract_address)?;

    let token_type = match token_type {
        Some(token_type) => token_type,
        None => Trait {
            trait_type: "any".to_string(),
            value: "any".to_string(),
        },
    };
    // load whitelist
    let whitelist_key = (
        owner,
        contract_address,
        format!("{}{}", token_type.trait_type, token_type.value),
    );
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

pub fn query_allowed_tokens(deps: Deps) -> StdResult<AllowedTokens> {
    let allowed_tokens = ALLOWED_TOKENS.load(deps.storage)?;
    Ok(allowed_tokens)
}
