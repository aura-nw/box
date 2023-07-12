#[cfg(test)]
pub mod env {
    use cosmwasm_std::{to_binary, Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    use crate::contract::{
        execute as VoucherExecute, instantiate as VoucherInstantiate, query as VoucherQuery,
    };

    use cw721_base::entry::{
        execute as NftExecute, instantiate as NftInstantiate, query as NftQuery,
    };

    // ****************************************
    // You MUST define the constants value here
    // ****************************************
    pub const ADMIN: &str = "aura1uh24g2lc8hvvkaaf7awz25lrh5fptthu2dhq0n";
    pub const USER_1: &str = "aura1fqj2redmssckrdeekhkcvd2kzp9f4nks4fctrt";

    pub const NATIVE_DENOM: &str = "uaura";
    pub const NATIVE_BALANCE: u128 = 1_000_000_000_000u128;

    pub struct ContractInfo {
        pub contract_addr: String,
        pub contract_code_id: u64,
    }

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(ADMIN),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(NATIVE_BALANCE),
                    }],
                )
                .unwrap();
        })
    }

    fn voucher_contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(VoucherExecute, VoucherInstantiate, VoucherQuery);
        Box::new(contract)
    }

    fn nft_contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(NftExecute, NftInstantiate, NftQuery);
        Box::new(contract)
    }

    // *********************************************************
    // You MUST store code and instantiate all contracts here
    // Follow the example (2) below:
    // @return App: the mock app
    // @return String: the address of the contract
    // @return u64: the code id of the contract
    //    pub fn instantiate_contracts() -> (App, String, u64) {
    //        // Create a new app instance
    //        let mut app = mock_app();
    //
    //        // store the code of all contracts to the app and get the code ids
    //        let contract_code_id = app.store_code(contract_template());
    //
    //        // create instantiate message for contract
    //        let contract_instantiate_msg = InstantiateMsg {
    //            name: "Contract_A".to_string(),
    //        };
    //
    //        // instantiate contract
    //        let contract_addr = app
    //            .instantiate_contract(
    //                contract_code_id,
    //                Addr::unchecked(ADMIN),
    //                &contract_instantiate_msg,
    //                &[],
    //                "test instantiate contract",
    //                None,
    //            )
    //            .unwrap();
    //
    //        // return the app instance, the addresses and code IDs of all contracts
    //        (app, contract_addr, contract_code_id)
    //    }
    // *********************************************************
    pub fn basic_instantiate_contracts() -> (App, Vec<ContractInfo>) {
        // Create a new app instance
        let mut app = mock_app();
        // Create a vector to store all contract info ([halo factory - [0], halo router - [1], cw20-base token - [2]])
        let mut contract_info_vec: Vec<ContractInfo> = Vec::new();

        // store code of all contracts to the app and get the code ids
        let voucher_contract_code_id = app.store_code(voucher_contract_template());
        let nft_contract_code_id = app.store_code(nft_contract_template());

        // nft contract
        // instantiate contract
        let nft_contract_addr = app
            .instantiate_contract(
                nft_contract_code_id,
                Addr::unchecked(ADMIN),
                &cw721_base::msg::InstantiateMsg {
                    name: "NFT".to_string(),
                    symbol: "NFT".to_string(),
                    minter: Addr::unchecked(ADMIN).to_string(),
                },
                &[],
                "test instantiate contract",
                None,
            )
            .unwrap();

        // add contract info to the vector
        contract_info_vec.push(ContractInfo {
            contract_addr: nft_contract_addr.to_string(),
            contract_code_id: nft_contract_code_id,
        });

        // voucher contract
        // instantiate contract
        let voucher_contract_addr = app
            .instantiate_contract(
                voucher_contract_code_id,
                Addr::unchecked(ADMIN),
                &crate::msg::InstantiateMsg {},
                &[],
                "test instantiate contract",
                None,
            )
            .unwrap();

        // add contract info to the vector
        contract_info_vec.push(ContractInfo {
            contract_addr: voucher_contract_addr.to_string(),
            contract_code_id: voucher_contract_code_id,
        });

        // return the app instance, the addresses and code IDs of all contracts
        (app, contract_info_vec)
    }

    pub fn instantiate_contracts() -> (App, Vec<ContractInfo>) {
        // Create a new app instance
        let mut app = mock_app();
        // Create a vector to store all contract info ([halo factory - [0], halo router - [1], cw20-base token - [2]])
        let mut contract_info_vec: Vec<ContractInfo> = Vec::new();

        // store code of all contracts to the app and get the code ids
        let voucher_contract_code_id = app.store_code(voucher_contract_template());
        let nft_contract_code_id = app.store_code(nft_contract_template());

        // nft contract
        // instantiate contract
        let nft_contract_addr = app
            .instantiate_contract(
                nft_contract_code_id,
                Addr::unchecked(ADMIN),
                &cw721_base::msg::InstantiateMsg {
                    name: "NFT".to_string(),
                    symbol: "NFT".to_string(),
                    minter: Addr::unchecked(ADMIN).to_string(),
                },
                &[],
                "test instantiate contract",
                None,
            )
            .unwrap();

        // add contract info to the vector
        contract_info_vec.push(ContractInfo {
            contract_addr: nft_contract_addr.to_string(),
            contract_code_id: nft_contract_code_id,
        });

        // voucher contract
        // instantiate contract
        let voucher_contract_addr = app
            .instantiate_contract(
                voucher_contract_code_id,
                Addr::unchecked(ADMIN),
                &crate::msg::InstantiateMsg {},
                &[],
                "test instantiate contract",
                None,
            )
            .unwrap();

        // add contract info to the vector
        contract_info_vec.push(ContractInfo {
            contract_addr: voucher_contract_addr.to_string(),
            contract_code_id: voucher_contract_code_id,
        });

        // nft contract FAKE
        // instantiate contract
        let nft_fake_contract_addr = app
            .instantiate_contract(
                nft_contract_code_id,
                Addr::unchecked(ADMIN),
                &cw721_base::msg::InstantiateMsg {
                    name: "NFT_FAKE".to_string(),
                    symbol: "NFTF".to_string(),
                    minter: Addr::unchecked(ADMIN).to_string(),
                },
                &[],
                "test instantiate contract",
                None,
            )
            .unwrap();

        // add contract info to the vector
        contract_info_vec.push(ContractInfo {
            contract_addr: nft_fake_contract_addr.to_string(),
            contract_code_id: nft_contract_code_id,
        });

        // create execute message for contract
        let execute_msg = crate::msg::ExecuteMsg::AllowToken {
            contract_address: nft_contract_addr.to_string(),
            token_type: None,
        };

        // execute contract
        let response = app.execute_contract(
            Addr::unchecked(ADMIN.to_string()),
            Addr::unchecked(voucher_contract_addr),
            &execute_msg,
            &[],
        );
        assert!(response.is_ok());

        // mint nft with real contract for USER_1
        let mint_msg = cw721_base::msg::ExecuteMsg::<Empty, Empty>::Mint {
            token_id: "1".to_string(),
            owner: USER_1.to_string(),
            token_uri: Some("Just a test".to_string()),
            extension: Empty::default(),
        };

        // execute contract
        let response = app.execute_contract(
            Addr::unchecked(ADMIN.to_string()),
            Addr::unchecked(nft_contract_addr.clone()),
            &mint_msg,
            &[],
        );
        assert!(response.is_ok());

        // mint nft with real contract for USER_1
        let mint_msg = cw721_base::msg::ExecuteMsg::<Empty, Empty>::Mint {
            token_id: "2".to_string(),
            owner: USER_1.to_string(),
            token_uri: Some("Just a test 2".to_string()),
            extension: Empty::default(),
        };

        // execute contract
        let response = app.execute_contract(
            Addr::unchecked(ADMIN.to_string()),
            Addr::unchecked(nft_contract_addr),
            &mint_msg,
            &[],
        );
        assert!(response.is_ok());

        // mint nft with fake contract for USER_1
        let mint_msg = cw721_base::msg::ExecuteMsg::<Empty, Empty>::Mint {
            token_id: "1".to_string(),
            owner: USER_1.to_string(),
            token_uri: Some("Just a test".to_string()),
            extension: Empty::default(),
        };

        // execute contract
        let response = app.execute_contract(
            Addr::unchecked(ADMIN.to_string()),
            Addr::unchecked(nft_fake_contract_addr),
            &mint_msg,
            &[],
        );
        assert!(response.is_ok());

        // return the app instance, the addresses and code IDs of all contracts
        (app, contract_info_vec)
    }

    // User can not allow nft contract is used in voucher contract
    #[test]
    pub fn cannot_allow_nft_contract_because_not_admin() {
        // instantiate all contracts
        let (mut app, contract_info_vec) = basic_instantiate_contracts();

        // get the contract info
        let voucher_contract_addr = &contract_info_vec[1].contract_addr;
        let nft_contract_addr = &contract_info_vec[0].contract_addr;

        // create execute message for contract
        let execute_msg = crate::msg::ExecuteMsg::AllowToken {
            contract_address: nft_contract_addr.to_string(),
            token_type: None,
        };

        // execute contract
        let response = app.execute_contract(
            Addr::unchecked(USER_1.to_string()),
            Addr::unchecked(voucher_contract_addr.clone()),
            &execute_msg,
            &[],
        );
        assert_eq!(
            response.unwrap_err().source().unwrap().to_string(),
            "Unauthorized"
        );
    }
    // Admin can allow nft contract is used in voucher contract
    #[test]
    pub fn admin_can_allow_nft_contract() {
        // instantiate all contracts
        let (mut app, contract_info_vec) = basic_instantiate_contracts();

        // get the contract info
        let voucher_contract_addr = &contract_info_vec[1].contract_addr;
        let nft_contract_addr = &contract_info_vec[0].contract_addr;

        // create execute message for contract
        let execute_msg = crate::msg::ExecuteMsg::AllowToken {
            contract_address: nft_contract_addr.to_string(),
            token_type: None,
        };

        // execute contract
        let response = app.execute_contract(
            Addr::unchecked(ADMIN.to_string()),
            Addr::unchecked(voucher_contract_addr.clone()),
            &execute_msg,
            &[],
        );
        assert!(response.is_ok());
    }

    // Cannot send fake nft to voucher contract
    #[test]
    fn cannot_send_fake_nft_to_voucher_contract() {
        // instantiate all contracts
        let (mut app, contract_info_vec) = instantiate_contracts();

        // get the contract info
        let voucher_contract_addr = &contract_info_vec[1].contract_addr;
        let nft_fake_contract_addr = &contract_info_vec[2].contract_addr;

        // create execute message for contract
        let execute_msg = cw721_base::msg::ExecuteMsg::<Empty, Empty>::SendNft {
            contract: voucher_contract_addr.to_string(),
            token_id: "1".to_string(),
            msg: to_binary(&crate::msg::Cw721HookMsg::Burn {
                contract_address: Addr::unchecked(nft_fake_contract_addr).to_string(),
                token_id: "1".to_string(),
                token_type: None,
            })
            .unwrap(),
        };

        // execute contract
        let response = app.execute_contract(
            Addr::unchecked(USER_1.to_string()),
            Addr::unchecked(nft_fake_contract_addr.clone()),
            &execute_msg,
            &[],
        );

        assert_eq!(
            response.unwrap_err().root_cause().to_string(),
            "Cannot burn this type of token"
        );
    }

    // Can send real nft to voucher contract
    #[test]
    fn cannot_send_real_nft_to_voucher_contract() {
        // instantiate all contracts
        let (mut app, contract_info_vec) = instantiate_contracts();

        // get the contract info
        let nft_contract_addr = &contract_info_vec[0].contract_addr;
        let voucher_contract_addr = &contract_info_vec[1].contract_addr;

        // create execute message for contract
        let execute_msg = cw721_base::msg::ExecuteMsg::<Empty, Empty>::SendNft {
            contract: voucher_contract_addr.to_string(),
            token_id: "1".to_string(),
            msg: to_binary(&crate::msg::Cw721HookMsg::Burn {
                contract_address: Addr::unchecked(nft_contract_addr).to_string(),
                token_id: "1".to_string(),
                token_type: None,
            })
            .unwrap(),
        };

        // execute contract
        let response = app.execute_contract(
            Addr::unchecked(USER_1.to_string()),
            Addr::unchecked(nft_contract_addr.clone()),
            &execute_msg,
            &[],
        );

        assert!(response.is_ok());

        // query RemainingVouchers of USER_1 in voucher contract
        let query_msg = crate::msg::QueryMsg::RemainingVouchers {
            owner: USER_1.to_string(),
            contract_address: nft_contract_addr.to_string(),
            token_type: None,
        };

        let response: u64 = app
            .wrap()
            .query_wasm_smart(voucher_contract_addr.to_string(), &query_msg)
            .unwrap();

        assert_eq!(response, 1);

        // query RemainingVouchers of ADMIN in voucher contract
        let query_msg = crate::msg::QueryMsg::RemainingVouchers {
            owner: ADMIN.to_string(),
            contract_address: nft_contract_addr.to_string(),
            token_type: None,
        };

        let response: u64 = app
            .wrap()
            .query_wasm_smart(voucher_contract_addr.to_string(), &query_msg)
            .unwrap();

        assert_eq!(response, 0);

        // create execute message for contract
        let execute_msg = cw721_base::msg::ExecuteMsg::<Empty, Empty>::SendNft {
            contract: voucher_contract_addr.to_string(),
            token_id: "2".to_string(),
            msg: to_binary(&crate::msg::Cw721HookMsg::Burn {
                contract_address: Addr::unchecked(nft_contract_addr).to_string(),
                token_id: "2".to_string(),
                token_type: None,
            })
            .unwrap(),
        };

        // execute contract
        let response = app.execute_contract(
            Addr::unchecked(USER_1.to_string()),
            Addr::unchecked(nft_contract_addr.clone()),
            &execute_msg,
            &[],
        );

        assert!(response.is_ok());

        // query RemainingVouchers of USER_1 in voucher contract
        let query_msg = crate::msg::QueryMsg::RemainingVouchers {
            owner: USER_1.to_string(),
            contract_address: nft_contract_addr.to_string(),
            token_type: None,
        };

        let response: u64 = app
            .wrap()
            .query_wasm_smart(voucher_contract_addr.to_string(), &query_msg)
            .unwrap();

        assert_eq!(response, 2);

        // query owner of token 1 in nft contract
        let query_msg = cw721_base::msg::QueryMsg::<Empty>::OwnerOf {
            token_id: "1".to_string(),
            include_expired: None,
        };

        let response: cw721::OwnerOfResponse = app
            .wrap()
            .query_wasm_smart(nft_contract_addr.to_string(), &query_msg)
            .unwrap();

        assert_eq!(response.owner, voucher_contract_addr.to_string());
    }
}
