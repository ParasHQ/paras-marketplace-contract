use near_sdk::AccountId;
use near_sdk::serde_json::json;
use near_sdk::json_types::{U128, U64};
use near_sdk_sim::{view, to_yocto, DEFAULT_GAS};
use near_contract_standards::non_fungible_token::Token;
use paras_marketplace_contract::MarketDataJson;


use crate::utils::{
    init, create_nft_and_mint_one, STORAGE_ADD_MARKET_DATA, STORAGE_APPROVE,
    GAS_BUY,
};

mod utils;

#[test]
fn test_new() {
    let (marketplace, _, treasury, _, _, _, _) = init();

    let treasury_id: AccountId = view!(marketplace.get_treasury()).unwrap_json();
    assert_eq!(treasury_id, treasury.account_id());
}

#[test]
fn test_add_market_data() {
    let (marketplace, nft, _, alice, bob, chandra, _) = init();

    //owner marketplace and nft-> alice
    //seller -> bob
    //buyer -> chandra
    //treasury -> treasury
    //royalty to 10 different account

    create_nft_and_mint_one(&nft, &alice, &bob, &chandra);
    let msg = &json!({"market_type":"sale","price": to_yocto("3").to_string(), "ft_token_id": "near"}).to_string();

    chandra.call(
        marketplace.account_id(),
        "storage_deposit",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_ADD_MARKET_DATA,
    ).assert_success();

    let initial_storage_usage = marketplace.account().unwrap().storage_usage;

    let outcome = chandra.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": format!("{}:{}", "1", "1"),
            "account_id": marketplace.valid_account_id(),
            "msg": msg,
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_APPROVE,
    );

    outcome.assert_success();
    let storage_price_for_add_market = 
        (marketplace.account().unwrap().storage_usage - initial_storage_usage) as u128 * 10u128.pow(19);
    //println!("{:?}", outcome.promise_results());
    println!("[ADD MARKET DATA] Gas burnt: {} TeraGas", outcome.gas_burnt() as f64 / 1e12);
    println!("[ADD MARKET DATA] Storage price : {} yoctoNEAR", storage_price_for_add_market);
}

#[test]
fn test_buy() {
    let (marketplace, nft, _, alice, bob, chandra, root) = init();


    create_nft_and_mint_one(&nft, &alice, &bob, &chandra);
    let msg = &json!({"market_type":"sale","price": to_yocto("3").to_string(), "ft_token_id": "near"}).to_string();

    chandra.call(
        marketplace.account_id(),
        "storage_deposit",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_ADD_MARKET_DATA,
    ).assert_success();

    chandra.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": format!("{}:{}", "1", "1"),
            "account_id": marketplace.valid_account_id(),
            "msg": msg,
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_APPROVE,
    ).assert_success();

    //buyer
    let buyer_person = root.create_user(
        "o".repeat(64),
        to_yocto("100")
    );

    let initial_storage_usage = marketplace.account().unwrap().storage_usage;

    let outcome =  buyer_person.call(
        marketplace.account_id(),
        "buy",
        &json!({
            "nft_contract_id": nft.account_id(),
            "token_id": format!("{}:{}", "1", "1"),
        }).to_string().into_bytes(),
        GAS_BUY,
        to_yocto("3"),
    );

    let restored_storage_price_for_buy = 
        initial_storage_usage - marketplace.account().unwrap().storage_usage;

    outcome.assert_success();
    println!(
        "tokens_burnt: {}Ⓝ",
        (outcome.tokens_burnt()) as f64 / 1e24
    );
    println!("[BUY] Gas burnt: {} TeraGas", outcome.gas_burnt() as f64 / 1e12);
    println!("[BUY] Restored storage price : {} Bytes", restored_storage_price_for_buy);
    let expected_gas_ceiling = 50 * u64::pow(10, 12);
    assert!(outcome.gas_burnt() < expected_gas_ceiling);
}

#[test]
fn test_add_offer() {
    let (marketplace, nft, _, alice, bob, chandra, _) = init();

    //owner marketplace and nft-> alice
    //seller -> bob
    //buyer -> chandra
    //treasury -> treasury
    //royalty to 10 different account

    create_nft_and_mint_one(&nft, &alice, &bob, &chandra);

    bob.call(
        marketplace.account_id(),
        "storage_deposit",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_ADD_MARKET_DATA,
    ).assert_success();

    let initial_storage_usage = marketplace.account().unwrap().storage_usage;

    let outcome = bob.call(
        marketplace.account_id(),
        "add_offer",
        &json!({
            "nft_contract_id": nft.account_id(),
            "token_id": "1:1",
            "ft_token_id": "near",
            "price": U128(10u128.pow(24)),
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    );

    outcome.assert_success();
    let storage_price =
        (marketplace.account().unwrap().storage_usage - initial_storage_usage) as u128 * 10u128.pow(19);

    println!(
        "tokens_burnt: {}Ⓝ",
        (outcome.tokens_burnt()) as f64 / 1e24
    );
    println!("[ADD_OFFER] Gas burnt: {} TeraGas", outcome.gas_burnt() as f64 / 1e12);
    println!("[ADD_OFFER] Storage price : {} N", storage_price);
}

#[test]
fn test_accept_offer() {
    let (marketplace, nft, _, alice, bob, chandra, _) = init();

    //owner marketplace and nft-> alice
    //seller -> bob
    //buyer -> chandra
    //treasury -> treasury
    //royalty to 10 different account

    create_nft_and_mint_one(&nft, &alice, &bob, &chandra);

    bob.call(
        marketplace.account_id(),
        "storage_deposit",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_ADD_MARKET_DATA,
    ).assert_success();

    bob.call(
        marketplace.account_id(),
        "add_offer",
        &json!({
            "nft_contract_id": nft.account_id(),
            "token_id": "1:1",
            "ft_token_id": "near",
            "price": U128(10u128.pow(24)),
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    ).assert_success();

    // chandra accept bid and nft_approve to marketplace
    let msg = &json!(
        {"market_type":"accept_offer","account_id": bob.account_id}).to_string();

    chandra.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": format!("{}:{}", "1", "1"),
            "account_id": marketplace.valid_account_id(),
            "msg": msg,
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_APPROVE,
    ).assert_success();

    let token: Token = nft.view(
        nft.account_id(),
        "nft_token",
        &json!({
            "token_id": "1:1"
        }).to_string().into_bytes()
    ).unwrap_json();

    assert_eq!(token.owner_id, bob.account_id());
}

#[test]
fn test_add_market_data_auction_timed() {
    let (marketplace, nft, _, alice, bob, chandra, _) = init();

    //owner marketplace and nft-> alice
    //seller -> bob
    //buyer -> chandra
    //treasury -> treasury
    //royalty to 10 different account

    const OCTOBER_1_2021: u64 = 1633046400000000000;
    const ONE_DAY: u64 = 86400000000000;

    create_nft_and_mint_one(&nft, &alice, &bob, &chandra);
    let msg = &json!({
        "market_type":"sale",
        "price": to_yocto("3").to_string(), 
        "ft_token_id": "near",
        "is_auction": true,
        "started_at": U64(OCTOBER_1_2021),
        "ended_at": U64(OCTOBER_1_2021 + ONE_DAY*7),
    }).to_string();

    chandra.call(
        marketplace.account_id(),
        "storage_deposit",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_ADD_MARKET_DATA,
    ).assert_success();

    let initial_storage_usage = marketplace.account().unwrap().storage_usage;

    let outcome = chandra.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": format!("{}:{}", "1", "1"),
            "account_id": marketplace.valid_account_id(),
            "msg": msg,
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_APPROVE,
    );

    outcome.assert_success();
    let storage_price_for_add_market = 
        (marketplace.account().unwrap().storage_usage - initial_storage_usage) as u128 * 10u128.pow(19);
    //println!("{:?}", outcome.promise_results());
    println!("[ADD MARKET DATA AUCTION] Gas burnt: {} TeraGas", outcome.gas_burnt() as f64 / 1e12);
    println!("[ADD MARKET DATA AUCTION] Storage price : {} yoctoNEAR", storage_price_for_add_market);

    alice.borrow_runtime_mut().cur_block.block_timestamp = OCTOBER_1_2021 - ONE_DAY;

    let outcome = alice.call(
        marketplace.account_id(),
        "add_bid",
        &json!({
            "nft_contract_id": nft.valid_account_id(),
            "token_id": format!("{}:{}", "1", "1"),
            "ft_token_id": "near",
            "amount": (to_yocto("3") + 2).to_string()
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        to_yocto("3") + 2
    );

    assert_eq!(outcome.promise_errors().len(), 1);

    alice.borrow_runtime_mut().cur_block.block_timestamp = OCTOBER_1_2021 + ONE_DAY;

    let outcome = alice.call(
        marketplace.account_id(),
        "add_bid",
        &json!({
            "nft_contract_id": nft.valid_account_id(),
            "token_id": format!("{}:{}", "1", "1"),
            "ft_token_id": "near",
            "amount": (to_yocto("3") + 1).to_string()
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        to_yocto("3") + 1
    );

    assert_eq!(outcome.promise_errors().len(), 0);

    let first_outcome = outcome.promise_results().remove(1).unwrap();
    println!("Outcome {:?}", first_outcome.logs()[0]);

    alice.borrow_runtime_mut().cur_block.block_timestamp = OCTOBER_1_2021 + ONE_DAY*9;

    let outcome = alice.call(
        marketplace.account_id(),
        "add_bid",
        &json!({
            "nft_contract_id": nft.valid_account_id(),
            "token_id": format!("{}:{}", "1", "1"),
            "ft_token_id": "near",
            "amount": (to_yocto("3") + 2).to_string()
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        to_yocto("3") + 2
    );

    assert_eq!(outcome.promise_errors().len(), 1);




    // let market_data: MarketDataJson = alice.view(
    //     marketplace.account_id(),
    //     "get_market_data",
    //     &json!({
    //         "nft_contract_id": nft.account_id(),
    //         "token_id": format!("{}:{}", "1", "1")
    //     }).to_string().into_bytes(),
    // ).unwrap_json();


    // println!("{:#?}", market_data);
}