use near_sdk::AccountId;
use near_sdk::serde_json::json;
use near_sdk_sim::{view, call, to_yocto, DEFAULT_GAS};

use crate::utils::{init, create_nft_and_mint_one, STORAGE_ADD_MARKET_DATA, STORAGE_APPROVE};
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
    let msg = &json!({"price": to_yocto("3").to_string(), "ft_token_id": "near"}).to_string();

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
            "token_id": format!("{}:{}", u128::MAX.to_string(), "1"),
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
    let (marketplace, nft, treasury, alice, bob, chandra, root) = init();


    create_nft_and_mint_one(&nft, &alice, &bob, &chandra);
    let msg = &json!({"price": to_yocto("3").to_string(), "ft_token_id": "near"}).to_string();

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
            "token_id": format!("{}:{}", u128::MAX.to_string(), "1"),
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
            "token_id": format!("{}:{}", u128::MAX.to_string(), "1"),
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        to_yocto("3"),
    );

    let restored_storage_price_for_buy = 
        initial_storage_usage - marketplace.account().unwrap().storage_usage;

    //println!("{:?}", outcome.promise_results());
    println!("[BUY] Gas burnt: {} TeraGas", outcome.gas_burnt() as f64 / 1e12);
    println!("[BUY] Restored storage price : {} Bytes", restored_storage_price_for_buy);
}