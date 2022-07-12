use cosmwasm_std::Addr;
use cw721_base::InstantiateMsg as NftInstantiateMsg;
use cw_multi_test::{App, Executor};

use account_nft::msg::ExecuteMsg as NftExecuteMsg;
use rover::adapters::{RedBankBase, RedBankUnchecked};
use rover::msg::{ExecuteMsg, InstantiateMsg};

use crate::helpers::{mock_account_nft_contract, mock_app, mock_contract, query_config};

pub mod helpers;

#[test]
fn test_update_config_works_with_full_config() {
    let mut app = mock_app();
    let original_owner = Addr::unchecked("original_owner");
    let code_id = app.store_code(mock_contract());
    let contract_addr = instantiate(&mut app, &original_owner, code_id);

    let config_res = query_config(&mut app, &contract_addr.clone());

    assert_eq!(config_res.account_nft, None);
    assert_eq!(config_res.owner, original_owner.to_string());

    let new_owner = Addr::unchecked("new_owner");
    let nft_contract_addr = setup_nft_and_propose_owner(&mut app, &original_owner, &contract_addr);
    let new_red_bank_addr = RedBankUnchecked {
        contract_addr: String::from("new_red_bank_addr"),
    };

    app.execute_contract(
        original_owner.clone(),
        contract_addr.clone(),
        &ExecuteMsg::UpdateConfig {
            account_nft: Some(nft_contract_addr.to_string()),
            owner: Some(new_owner.to_string()),
            red_bank: Some(new_red_bank_addr.clone()),
        },
        &[],
    )
    .unwrap();

    let config_res = query_config(&mut app, &contract_addr.clone());

    assert_eq!(config_res.account_nft, Some(nft_contract_addr.to_string()));
    assert_eq!(config_res.owner, new_owner.to_string());
    assert_eq!(config_res.red_bank, new_red_bank_addr.contract_addr);
}

#[test]
fn test_update_config_works_with_some_config() {
    let mut app = mock_app();
    let original_owner = Addr::unchecked("original_owner");
    let code_id = app.store_code(mock_contract());
    let contract_addr = instantiate(&mut app, &original_owner, code_id);

    let config_res = query_config(&mut app, &contract_addr.clone());

    assert_eq!(config_res.account_nft, None);
    assert_eq!(config_res.owner, original_owner.to_string());
    assert_eq!(config_res.red_bank, String::from("initial_red_bank"));

    let nft_contract_addr = setup_nft_and_propose_owner(&mut app, &original_owner, &contract_addr);
    app.execute_contract(
        original_owner.clone(),
        contract_addr.clone(),
        &ExecuteMsg::UpdateConfig {
            account_nft: Some(nft_contract_addr.to_string()),
            owner: None,
            red_bank: None,
        },
        &[],
    )
    .unwrap();

    let config_res = query_config(&mut app, &contract_addr.clone());

    assert_eq!(config_res.account_nft, Some(nft_contract_addr.to_string()));
    assert_eq!(config_res.owner, original_owner.to_string());
    assert_eq!(config_res.red_bank, String::from("initial_red_bank"));

    let new_owner = Addr::unchecked("new_owner");
    app.execute_contract(
        original_owner.clone(),
        contract_addr.clone(),
        &ExecuteMsg::UpdateConfig {
            account_nft: None,
            owner: Some(new_owner.to_string()),
            red_bank: None,
        },
        &[],
    )
    .unwrap();

    let config_res = query_config(&mut app, &contract_addr.clone());
    assert_eq!(config_res.account_nft, Some(nft_contract_addr.to_string()));
    assert_eq!(config_res.owner, new_owner.to_string());
    assert_eq!(config_res.red_bank, String::from("initial_red_bank"));

    let new_red_bank = RedBankUnchecked {
        contract_addr: String::from("new_red_bank_addr"),
    };
    app.execute_contract(
        new_owner.clone(),
        contract_addr.clone(),
        &ExecuteMsg::UpdateConfig {
            account_nft: None,
            owner: None,
            red_bank: Some(new_red_bank.clone()),
        },
        &[],
    )
    .unwrap();

    let config_res = query_config(&mut app, &contract_addr.clone());
    assert_eq!(config_res.account_nft, Some(nft_contract_addr.to_string()));
    assert_eq!(config_res.owner, new_owner.to_string());
    assert_eq!(config_res.red_bank, new_red_bank.contract_addr);
}

#[test]
fn test_update_config_does_nothing_when_nothing_is_passed() {
    let mut app = mock_app();
    let original_owner = Addr::unchecked("original_owner");
    let code_id = app.store_code(mock_contract());
    let contract_addr = instantiate(&mut app, &original_owner, code_id);

    app.execute_contract(
        original_owner.clone(),
        contract_addr.clone(),
        &ExecuteMsg::UpdateConfig {
            account_nft: None,
            owner: None,
            red_bank: None,
        },
        &[],
    )
    .unwrap();

    let config_res = query_config(&mut app, &contract_addr.clone());

    assert_eq!(config_res.account_nft, None);
    assert_eq!(config_res.owner, original_owner.to_string());
}

fn instantiate(app: &mut App, original_owner: &Addr, code_id: u64) -> Addr {
    app.instantiate_contract(
        code_id,
        original_owner.clone(),
        &InstantiateMsg {
            owner: original_owner.to_string(),
            allowed_vaults: vec![],
            allowed_assets: vec![],
            red_bank: RedBankBase {
                contract_addr: String::from("initial_red_bank"),
            },
        },
        &[],
        "mock_manager_contract",
        None,
    )
    .unwrap()
}

fn setup_nft_and_propose_owner(app: &mut App, original_owner: &Addr, contract_addr: &Addr) -> Addr {
    let nft_contract_code_id = app.store_code(mock_account_nft_contract());
    let nft_contract_addr = app
        .instantiate_contract(
            nft_contract_code_id,
            original_owner.clone(),
            &NftInstantiateMsg {
                name: "Rover Credit Account".to_string(),
                symbol: "RCA".to_string(),
                minter: original_owner.to_string(),
            },
            &[],
            "manager-mock-account-nft",
            None,
        )
        .unwrap();

    let proposal_msg: NftExecuteMsg = NftExecuteMsg::ProposeNewOwner {
        new_owner: contract_addr.to_string(),
    };
    app.execute_contract(
        original_owner.clone(),
        nft_contract_addr.clone(),
        &proposal_msg,
        &[],
    )
    .unwrap();
    nft_contract_addr
}
