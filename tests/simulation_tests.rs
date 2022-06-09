use near_contract_standards::non_fungible_token::Token;
use near_sdk::{
    json_types::{U128, U64},
    serde::{Deserialize, Serialize},
    serde_json::json,
    AccountId, Gas,
};
use near_sdk_sim::{to_yocto, view};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid {
    pub bidder_id: AccountId,
    pub price: U128,
}

pub type Bids = Vec<Bid>;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MarketDataJson {
    owner_id: AccountId,
    approval_id: U64,
    nft_contract_id: String,
    token_id: String,
    ft_token_id: AccountId, // "near" for NEAR token
    price: U128,
    bids: Option<Bids>,
    started_at: Option<U64>,
    ended_at: Option<U64>,
    end_price: Option<U128>, // dutch auction
    is_auction: Option<bool>,
}

use crate::utils::{
    account_o, create_nft_and_mint_one, init, DEFAULT_GAS, GAS_BUY, STORAGE_ADD_MARKET_DATA,
    STORAGE_APPROVE,
};

mod utils;

#[test]
fn test_new() {
    let (marketplace, _, treasury, _, _, _, _, _) = init();

    let treasury_id: AccountId = view!(marketplace.get_treasury()).unwrap_json();
    assert_eq!(treasury_id, treasury.account_id());
}

#[test]
fn test_add_market_data() {
    let (marketplace, nft, _, alice, bob, chandra, darmaji, _) = init();

    //owner marketplace and nft-> alice
    //seller -> bob
    //buyer -> chandra
    //treasury -> treasury
    //royalty to 10 different account

    create_nft_and_mint_one(&nft, &alice, &bob, &chandra, &darmaji);
    let msg =
        &json!({"market_type":"sale","price": to_yocto("3").to_string(), "ft_token_id": "near"})
            .to_string();

    chandra
        .call(
            marketplace.account_id(),
            "storage_deposit",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            STORAGE_ADD_MARKET_DATA,
        )
        .assert_success();

    let initial_storage_usage = marketplace.account().unwrap().storage_usage;

    let outcome = chandra.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": format!("{}:{}", "1", "1"),
            "account_id": marketplace.account_id(),
            "msg": msg,
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        STORAGE_APPROVE,
    );

    outcome.assert_success();
    let storage_price_for_add_market = (marketplace.account().unwrap().storage_usage
        - initial_storage_usage) as u128
        * 10u128.pow(19);
    //println!("{:?}", outcome.promise_results());
    println!(
        "[ADD MARKET DATA] Gas burnt: {} TeraGas",
        outcome.gas_burnt().0 as f64 / 1e12
    );
    println!(
        "[ADD MARKET DATA] Storage price : {} yoctoNEAR",
        storage_price_for_add_market
    );
}
#[test]
fn test_sale_with_sucess_add_trade_storage(){
    /*
    1. user deposit storage
    2. user add market sale
    3. user deposit storage
    4. user add trade
    5. other user buy the nft
     */
    let (marketplace, nft, _, alice, bob, chandra, darmaji, root) = init();

    create_nft_and_mint_one(&nft, &alice, &bob, &chandra, &darmaji);
    let msg =
        &json!({"market_type":"sale","price": to_yocto("3").to_string(), "ft_token_id": "near"})
            .to_string();

    chandra
        .call(
            marketplace.account_id(),
            "storage_deposit",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            STORAGE_ADD_MARKET_DATA,
        )
        .assert_success();


    chandra
        .call(
            nft.account_id(),
            "nft_approve",
            &json!({
                "token_id": "1:1",
                "account_id": marketplace.account_id(),
                "msg": msg,
            })
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            STORAGE_APPROVE,
        )
        .assert_success();

    chandra
        .call(
            marketplace.account_id(),
            "storage_deposit",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            STORAGE_ADD_MARKET_DATA,
        )
        .assert_success();

    // success add trade
    chandra.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": "1:1",
            "account_id": marketplace.account_id(),
            "msg": &json!{{
                "market_type": "add_trade",
                "seller_nft_contract_id": nft.account_id(),
                "seller_token_id": "1:2",
            }}.to_string()
        })
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    ).assert_success();

    //buyer
    let buyer_person = root.create_user(account_o(), to_yocto("100"));

    let initial_storage_usage = marketplace.account().unwrap().storage_usage;

    let outcome = buyer_person.call(
        marketplace.account_id(),
        "buy",
        &json!({
            "nft_contract_id": nft.account_id(),
            "token_id": format!("{}:{}", "1", "1"),
        })
            .to_string()
            .into_bytes(),
        GAS_BUY,
        to_yocto("3"),
    );

    let restored_storage_price_for_buy =
        initial_storage_usage - marketplace.account().unwrap().storage_usage;

    println!("tokens_burnt: {}Ⓝ", (outcome.tokens_burnt()) as f64 / 1e24);
    println!(
        "[BUY] Gas burnt: {} TeraGas",
        outcome.gas_burnt().0 as f64 / 1e12
    );
    outcome.assert_success();
    println!(
        "[BUY] Restored storage price : {} Bytes",
        restored_storage_price_for_buy
    );
    let expected_gas_ceiling = 50 * u64::pow(10, 12);
    assert!(outcome.gas_burnt() < Gas(expected_gas_ceiling));
}
#[test]
fn test_sale_with_fail_add_trade_storage(){
    /*
    1. user deposit storage
    2. user add market sale
    3. user doesn't deposit storage
    4. user add trade
    5. other user check that trade from step 4 does not exist
    6. other user buy the nft
     */
    let (marketplace, nft, _, alice, bob, chandra, darmaji, root) = init();

    create_nft_and_mint_one(&nft, &alice, &bob, &chandra, &darmaji);
    let msg =
        &json!({"market_type":"sale","price": to_yocto("3").to_string(), "ft_token_id": "near"})
            .to_string();

    chandra
        .call(
            marketplace.account_id(),
            "storage_deposit",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            STORAGE_ADD_MARKET_DATA,
        )
        .assert_success();


    chandra
        .call(
            nft.account_id(),
            "nft_approve",
            &json!({
                "token_id": "1:1",
                "account_id": marketplace.account_id(),
                "msg": msg,
            })
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            STORAGE_APPROVE,
        )
        .assert_success();

    // failed add trade due cause already approved
    chandra.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": "1:1",
            "account_id": marketplace.account_id(),
            "msg": &json!{{
                "market_type": "add_trade",
                "seller_nft_contract_id": nft.account_id(),
                "seller_token_id": "1:2",
            }}.to_string()
        })
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    );

    // no trade exist for current user
    darmaji.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": "1:2",
            "account_id": marketplace.account_id(),
            "msg": &json!{{
                "market_type": "accept_trade_paras_series",
                "buyer_id": chandra.account_id(),
                "buyer_nft_contract_id": nft.account_id(),
                "buyer_token_id": "1:1"
            }}.to_string()
        })
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    );

    //buyer
    let buyer_person = root.create_user(account_o(), to_yocto("100"));

    let initial_storage_usage = marketplace.account().unwrap().storage_usage;

    let outcome = buyer_person.call(
        marketplace.account_id(),
        "buy",
        &json!({
            "nft_contract_id": nft.account_id(),
            "token_id": format!("{}:{}", "1", "1"),
        })
            .to_string()
            .into_bytes(),
        GAS_BUY,
        to_yocto("3"),
    );

    let restored_storage_price_for_buy =
        initial_storage_usage - marketplace.account().unwrap().storage_usage;

    println!("tokens_burnt: {}Ⓝ", (outcome.tokens_burnt()) as f64 / 1e24);
    println!(
        "[BUY] Gas burnt: {} TeraGas",
        outcome.gas_burnt().0 as f64 / 1e12
    );
    outcome.assert_success();
    println!(
        "[BUY] Restored storage price : {} Bytes",
        restored_storage_price_for_buy
    );
    let expected_gas_ceiling = 50 * u64::pow(10, 12);
    assert!(outcome.gas_burnt() < Gas(expected_gas_ceiling));
}

#[test]
fn test_multiple_add_trade_with_one_failed_trade(){
    /*
    1. user deposit storage
    2. user A add trade NFT 1 with NFT 2
    3. user A doesn't deposit storage
    4. user A add trade NFT 1 with NFT 3
    5. other user check that trade NFT 1 with NFT 3 does not exist
    6. user B accept trade NFT 2 with NFT 1 (from step 2)
     */
    let (marketplace, nft, _, alice, bob, chandra, darmaji, root) = init();

    create_nft_and_mint_one(&nft, &alice, &bob, &chandra, &darmaji);

    chandra
        .call(
            marketplace.account_id(),
            "storage_deposit",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            STORAGE_ADD_MARKET_DATA,
        )
        .assert_success();

    chandra.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": "1:1",
            "account_id": marketplace.account_id(),
            "msg": &json!{{
                "market_type": "add_trade",
                "seller_nft_contract_id": nft.account_id(),
                "seller_token_id": "1:2",
            }}.to_string()
        })
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    ).assert_success();

    //fail add trade
    chandra.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": "1:1",
            "account_id": marketplace.account_id(),
            "msg": &json!{{
                "market_type": "add_trade",
                "seller_nft_contract_id": nft.account_id(),
                "seller_token_id": "1:3",
            }}.to_string()
        })
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    );

    //no trade data for 1 => 3
    // darmaji.call(
    //     marketplace.account_id(),
    //     "storage_deposit",
    //     &json!({}).to_string().into_bytes(),
    //     DEFAULT_GAS,
    //     STORAGE_ADD_MARKET_DATA,
    // )
    //     .assert_success();
    //
    darmaji.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": "1:3",
            "account_id": marketplace.account_id(),
            "msg": &json!{{
                "market_type": "accept_trade",
                "buyer_id": chandra.account_id(),
                "buyer_nft_contract_id": nft.account_id(),
                "buyer_token_id": "1:1"
            }}.to_string()
        })
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    );

    //accept trade 1 => 2
    darmaji.call(
        marketplace.account_id(),
        "storage_deposit",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_ADD_MARKET_DATA,
    )
        .assert_success();

    darmaji.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": "1:2",
            "account_id": marketplace.account_id(),
            "msg": &json!{{
                "market_type": "accept_trade",
                "buyer_id": chandra.account_id(),
                "buyer_nft_contract_id": nft.account_id(),
                "buyer_token_id": "1:1"
            }}.to_string()
        })
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    ).assert_success();

    let chandra_token: Token = nft
        .view(
            nft.account_id(),
            "nft_token",
            &json!({
                "token_id": "1:1"
            })
                .to_string()
                .into_bytes(),
        )
        .unwrap_json();

    let darmaji_token: Token = nft
        .view(
            nft.account_id(),
            "nft_token",
            &json!({
                "token_id": "1:2"
            })
                .to_string()
                .into_bytes(),
        )
        .unwrap_json();

    let darmaji_token2: Token = nft
        .view(
            nft.account_id(),
            "nft_token",
            &json!({
                "token_id": "1:3"
            })
                .to_string()
                .into_bytes(),
        )
        .unwrap_json();
    println!("1:1 Token Owner {}",chandra_token.owner_id);
    println!("1:2 Token Owner {}",darmaji_token.owner_id);
    println!("1:3 Token Owner {}",darmaji_token2.owner_id);
    assert_eq!(chandra_token.owner_id, darmaji.account_id());
    assert_eq!(darmaji_token.owner_id, chandra.account_id());
    assert_eq!(darmaji_token2.owner_id, darmaji.account_id());
}

#[test]
fn test_add_trade_with_fail_sale(){
    /*
    1. user deposit storage
    2. user A add trade NFT 1 with NFT 2
    3. user A doesn't deposit storage
    4. user A add sale
    5. other user check that sale NFT 1 does not exist
    6. user B accept trade NFT 2 with NFT 1 (from step 2)
     */
    let (marketplace, nft, _, alice, bob, chandra, darmaji, root) = init();

    create_nft_and_mint_one(&nft, &alice, &bob, &chandra, &darmaji);

    chandra
        .call(
            marketplace.account_id(),
            "storage_deposit",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            STORAGE_ADD_MARKET_DATA,
        )
        .assert_success();

    chandra.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": "1:1",
            "account_id": marketplace.account_id(),
            "msg": &json!{{
                "market_type": "add_trade",
                "seller_nft_contract_id": nft.account_id(),
                "seller_token_id": "1:2",
            }}.to_string()
        })
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    ).assert_success();

    //add fail sale
    let outcome_fail_sale = chandra
        .call(
            nft.account_id(),
            "nft_approve",
            &json!({
                "token_id": format!("{}:{}", "1", "1"),
                "account_id": marketplace.account_id(),
                "msg": &json!({"market_type":"sale","price": to_yocto("3").to_string(), "ft_token_id": "near"}).to_string(),
            })
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            STORAGE_APPROVE,
        );

    println!("{:?}", outcome_fail_sale.promise_errors());

    //check fail sale
    darmaji.call(
        marketplace.account_id(),
        "buy",
        &json!({
            "nft_contract_id": nft.account_id(),
            "token_id": format!("{}:{}", "1", "1"),
        })
            .to_string()
            .into_bytes(),
        GAS_BUY,
        to_yocto("3"),
    );
    //accept trade 1 => 2
    darmaji.call(
        marketplace.account_id(),
        "storage_deposit",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_ADD_MARKET_DATA,
    )
        .assert_success();

    let outcome = darmaji.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": "1:2",
            "account_id": marketplace.account_id(),
            "msg": &json!{{
                "market_type": "accept_trade",
                "buyer_id": chandra.account_id(),
                "buyer_nft_contract_id": nft.account_id(),
                "buyer_token_id": "1:1"
            }}.to_string()
        })
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    );

    println!("{:?}", outcome.promise_errors());

    let chandra_token: Token = nft
        .view(
            nft.account_id(),
            "nft_token",
            &json!({
                "token_id": "1:1"
            })
                .to_string()
                .into_bytes(),
        )
        .unwrap_json();

    let darmaji_token: Token = nft
        .view(
            nft.account_id(),
            "nft_token",
            &json!({
                "token_id": "1:2"
            })
                .to_string()
                .into_bytes(),
        )
        .unwrap_json();

    println!("1:1 Token Owner {}",chandra_token.owner_id);
    println!("1:2 Token Owner {}",darmaji_token.owner_id);
    assert_eq!(chandra_token.owner_id, darmaji.account_id());
    assert_eq!(darmaji_token.owner_id, chandra.account_id());
}
#[test]
fn test_buy() {
    let (marketplace, nft, _, alice, bob, chandra, darmaji, root) = init();

    create_nft_and_mint_one(&nft, &alice, &bob, &chandra, &darmaji);
    let msg =
        &json!({"market_type":"sale","price": to_yocto("3").to_string(), "ft_token_id": "near"})
            .to_string();

    chandra
        .call(
            marketplace.account_id(),
            "storage_deposit",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            STORAGE_ADD_MARKET_DATA,
        )
        .assert_success();

    chandra
        .call(
            nft.account_id(),
            "nft_approve",
            &json!({
                "token_id": format!("{}:{}", "1", "1"),
                "account_id": marketplace.account_id(),
                "msg": msg,
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            STORAGE_APPROVE,
        )
        .assert_success();

    //buyer
    let buyer_person = root.create_user(account_o(), to_yocto("100"));

    let initial_storage_usage = marketplace.account().unwrap().storage_usage;

    let outcome = buyer_person.call(
        marketplace.account_id(),
        "buy",
        &json!({
            "nft_contract_id": nft.account_id(),
            "token_id": format!("{}:{}", "1", "1"),
        })
        .to_string()
        .into_bytes(),
        GAS_BUY,
        to_yocto("3"),
    );

    let restored_storage_price_for_buy =
        initial_storage_usage - marketplace.account().unwrap().storage_usage;

    println!("tokens_burnt: {}Ⓝ", (outcome.tokens_burnt()) as f64 / 1e24);
    println!(
        "[BUY] Gas burnt: {} TeraGas",
        outcome.gas_burnt().0 as f64 / 1e12
    );
    outcome.assert_success();
    println!(
        "[BUY] Restored storage price : {} Bytes",
        restored_storage_price_for_buy
    );
    let expected_gas_ceiling = 50 * u64::pow(10, 12);
    assert!(outcome.gas_burnt() < Gas(expected_gas_ceiling));
}

#[test]
fn test_add_offer() {
    let (marketplace, nft, _, alice, bob, chandra, darmaji, _) = init();

    //owner marketplace and nft-> alice
    //seller -> bob
    //buyer -> chandra
    //treasury -> treasury
    //royalty to 10 different account

    create_nft_and_mint_one(&nft, &alice, &bob, &chandra, &darmaji);

    bob.call(
        marketplace.account_id(),
        "storage_deposit",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_ADD_MARKET_DATA,
    )
    .assert_success();

    let initial_storage_usage = marketplace.account().unwrap().storage_usage;

    let outcome = bob.call(
        marketplace.account_id(),
        "add_offer",
        &json!({
            "nft_contract_id": nft.account_id(),
            "token_id": "1:1",
            "ft_token_id": "near",
            "price": U128(10u128.pow(24)),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    );

    outcome.assert_success();
    let storage_price = (marketplace.account().unwrap().storage_usage - initial_storage_usage)
        as u128
        * 10u128.pow(19);

    println!("tokens_burnt: {}Ⓝ", (outcome.tokens_burnt()) as f64 / 1e24);
    println!(
        "[ADD_OFFER] Gas burnt: {} TeraGas",
        outcome.gas_burnt().0 as f64 / 1e12
    );
    println!("[ADD_OFFER] Storage price : {} N", storage_price);
}

#[test]
fn test_accept_offer() {
    let (marketplace, nft, _, alice, bob, chandra, darmaji, _) = init();

    //owner marketplace and nft-> alice
    //seller -> bob
    //buyer -> chandra
    //treasury -> treasury
    //royalty to 10 different account

    create_nft_and_mint_one(&nft, &alice, &bob, &chandra, &darmaji);

    bob.call(
        marketplace.account_id(),
        "storage_deposit",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_ADD_MARKET_DATA,
    )
    .assert_success();

    bob.call(
        marketplace.account_id(),
        "add_offer",
        &json!({
            "nft_contract_id": nft.account_id(),
            "token_id": "1:1",
            "ft_token_id": "near",
            "price": U128(10u128.pow(24)),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    )
    .assert_success();

    // chandra accept bid and nft_approve to marketplace
    let msg = &json!(
        {"market_type":"accept_offer","buyer_id": bob.account_id, "price": U128(10u128.pow(24))})
    .to_string();

    chandra
        .call(
            nft.account_id(),
            "nft_approve",
            &json!({
                "token_id": format!("{}:{}", "1", "1"),
                "account_id": marketplace.account_id(),
                "msg": msg,
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            STORAGE_APPROVE,
        )
        .assert_success();

    let token: Token = nft
        .view(
            nft.account_id(),
            "nft_token",
            &json!({
                "token_id": "1:1"
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();

    assert_eq!(token.owner_id, bob.account_id());
}

#[test]
fn test_accept_offer_paras_series() {
    let (marketplace, nft, _, alice, bob, chandra, darmaji, _) = init();

    //owner marketplace and nft-> alice
    //seller -> bob
    //buyer -> chandra
    //treasury -> treasury
    //royalty to 10 different account

    create_nft_and_mint_one(&nft, &alice, &bob, &chandra, &darmaji);

    bob.call(
        marketplace.account_id(),
        "storage_deposit",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_ADD_MARKET_DATA,
    )
    .assert_success();

    bob.call(
        marketplace.account_id(),
        "add_offer",
        &json!({
            "nft_contract_id": nft.account_id(),
            "token_series_id": "1",
            "ft_token_id": "near",
            "price": U128(10u128.pow(24)),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    )
    .assert_success();

    // chandra accept bid and nft_approve to marketplace
    let msg = &json!(
        {"market_type":"accept_offer_paras_series","buyer_id": bob.account_id, "price": U128(10u128.pow(24))}).to_string();

    chandra
        .call(
            nft.account_id(),
            "nft_approve",
            &json!({
                "token_id": format!("{}:{}", "1", "1"),
                "account_id": marketplace.account_id(),
                "msg": msg,
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            STORAGE_APPROVE,
        )
        .assert_success();

    let token: Token = nft
        .view(
            nft.account_id(),
            "nft_token",
            &json!({
                "token_id": "1:1"
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();

    assert_eq!(token.owner_id, bob.account_id());
}

// //trade

#[test]
fn test_add_trade() {
    let (marketplace, nft, _, alice, bob, chandra, darmaji, _) = init();
    create_nft_and_mint_one(&nft, &alice, &bob, &chandra, &darmaji);

    //chadra's token_id = 1:1
    //darmaji's token_id = 1:2

    chandra.call(
        marketplace.account_id(),
        "storage_deposit",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_ADD_MARKET_DATA,
    )
    .assert_success();

    let initial_storage_usage = marketplace.account().unwrap().storage_usage;

    let outcome = chandra.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": "1:1",
            "account_id": marketplace.account_id(),
            "msg": &json!{{
                "market_type": "add_trade",
                "seller_nft_contract_id": nft.account_id(),
                "seller_token_id": "1:2",
            }}.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    );

    outcome.assert_success();
    let storage_price = (marketplace.account().unwrap().storage_usage - initial_storage_usage)
        as u128
        * 10u128.pow(19);

    println!("tokens_burnt: {}Ⓝ", (outcome.tokens_burnt()) as f64 / 1e24);
    println!(
        "[ADD_TRADE] Gas burnt: {} TeraGas",
        outcome.gas_burnt().0 as f64 / 1e12
    );
    println!("[ADD_TRADE] Storage price : {} N", storage_price);
}

#[test]
fn test_accept_trade(){
    let (marketplace, nft, _, alice, bob, chandra, darmaji, _) = init();
    create_nft_and_mint_one(&nft, &alice, &bob, &chandra, &darmaji);

    //init
    //chadra's token_id = 1:1
    //darmaji's token_id = 1:2

    chandra.call(
        marketplace.account_id(),
        "storage_deposit",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_ADD_MARKET_DATA * 2,
    )
    .assert_success();

    chandra.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": "1:1",
            "account_id": marketplace.account_id(),
            "msg": &json!{{
                "market_type": "add_trade",
                "seller_nft_contract_id": nft.account_id(),
                "seller_token_id": "1:2",
            }}.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    ).assert_success();

    darmaji.call(
        marketplace.account_id(),
        "storage_deposit",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_ADD_MARKET_DATA,
    )
    .assert_success();

    darmaji.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": "1:2",
            "account_id": marketplace.account_id(),
            "msg": &json!{{
                "market_type": "accept_trade",
                "buyer_id": chandra.account_id(),
                "buyer_nft_contract_id": nft.account_id(),
                "buyer_token_id": "1:1"
            }}.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    ).assert_success();

    chandra.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": "1:2",
            "account_id": marketplace.account_id(),
            "msg": &json!{{
                "market_type": "add_trade",
                "seller_nft_contract_id": nft.account_id(),
                "seller_token_id": "1:1",
            }}.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    ).assert_success();

    darmaji.call(
        marketplace.account_id(),
        "storage_deposit",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_ADD_MARKET_DATA,
    )
    .assert_success();

    //after chandra trade his nft the result should be
    //chadra's token_id = 1:2
    //darmaji's token_id = 1:1
   
    let chandra_token: Token = nft
        .view(
            nft.account_id(),
            "nft_token",
            &json!({
                "token_id": "1:1"
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();

    let darmaji_token: Token = nft
        .view(
            nft.account_id(),
            "nft_token",
            &json!({
                "token_id": "1:2"
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();
   
    assert_eq!(chandra_token.owner_id, darmaji.account_id());
    assert_eq!(darmaji_token.owner_id, chandra.account_id());
}

#[test]
fn test_accept_trade_paras_series(){
    let (marketplace, nft, _, alice, bob, chandra, darmaji, _) = init();
    create_nft_and_mint_one(&nft, &alice, &bob, &chandra, &darmaji);

    //init
    //chadra's token_id = 1:1
    //darmaji's token_id = 1:2

    chandra.call(
        marketplace.account_id(),
        "storage_deposit",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_ADD_MARKET_DATA,
    )
    .assert_success();

    chandra.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": "1:1",
            "account_id": marketplace.account_id(),
            "msg": &json!{{
                "market_type": "add_trade",
                "seller_nft_contract_id": nft.account_id(),
                "seller_token_series_id": "1"
            }}.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    );

    darmaji.call(
        marketplace.account_id(),
        "storage_deposit",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_ADD_MARKET_DATA,
    )
    .assert_success();

    darmaji.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": "1:2",
            "account_id": marketplace.account_id(),
            "msg": &json!{{
                "market_type": "accept_trade_paras_series",
                "buyer_id": chandra.account_id(),
                "buyer_nft_contract_id": nft.account_id(),
                "buyer_token_id": "1:1"
            }}.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        10u128.pow(24),
    ).assert_success();

    //after chandra trade his nft the result should be
    //chadra's token_id = 1:2
    //darmaji's token_id = 1:1
   
    let chandra_token: Token = nft
        .view(
            nft.account_id(),
            "nft_token",
            &json!({
                "token_id": "1:1"
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();

    let darmaji_token: Token = nft
        .view(
            nft.account_id(),
            "nft_token",
            &json!({
                "token_id": "1:2"
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();
   
    assert_eq!(chandra_token.owner_id, darmaji.account_id());
    assert_eq!(darmaji_token.owner_id, chandra.account_id());
}

#[test]
fn test_add_market_data_auction_timed() {
    let (marketplace, nft, _, alice, bob, chandra, darmaji, _) = init();

    //owner marketplace and nft-> alice
    //seller -> bob
    //buyer -> chandra
    //treasury -> treasury
    //royalty to 10 different account

    const OCTOBER_1_2021: u64 = 1633046400000000000;
    const ONE_DAY: u64 = 86400000000000;

    create_nft_and_mint_one(&nft, &alice, &bob, &chandra, &darmaji);
    let msg = &json!({
        "market_type":"sale",
        "price": to_yocto("3").to_string(),
        "ft_token_id": "near",
        "is_auction": true,
        "started_at": U64(OCTOBER_1_2021),
        "ended_at": U64(OCTOBER_1_2021 + ONE_DAY*7),
    })
    .to_string();

    chandra
        .call(
            marketplace.account_id(),
            "storage_deposit",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            STORAGE_ADD_MARKET_DATA,
        )
        .assert_success();

    let initial_storage_usage = marketplace.account().unwrap().storage_usage;

    let outcome = chandra.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": format!("{}:{}", "1", "1"),
            "account_id": marketplace.account_id(),
            "msg": msg,
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        STORAGE_APPROVE,
    );

    outcome.assert_success();
    let storage_price_for_add_market = (marketplace.account().unwrap().storage_usage
        - initial_storage_usage) as u128
        * 10u128.pow(19);
    //println!("{:?}", outcome.promise_results());
    println!(
        "[ADD MARKET DATA AUCTION] Gas burnt: {} TeraGas",
        outcome.gas_burnt().0 as f64 / 1e12
    );
    println!(
        "[ADD MARKET DATA AUCTION] Storage price : {} yoctoNEAR",
        storage_price_for_add_market
    );

    alice.borrow_runtime_mut().cur_block.block_timestamp = OCTOBER_1_2021 - ONE_DAY;

    let outcome = alice.call(
        marketplace.account_id(),
        "add_bid",
        &json!({
            "nft_contract_id": nft.account_id(),
            "token_id": format!("{}:{}", "1", "1"),
            "ft_token_id": "near",
            "amount": (to_yocto("3") + 2).to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        to_yocto("3") + 2,
    );

    assert_eq!(outcome.promise_errors().len(), 1);

    alice.borrow_runtime_mut().cur_block.block_timestamp = OCTOBER_1_2021 + ONE_DAY;

    let outcome = alice.call(
        marketplace.account_id(),
        "add_bid",
        &json!({
            "nft_contract_id": nft.account_id(),
            "token_id": format!("{}:{}", "1", "1"),
            "ft_token_id": "near",
            "amount": (to_yocto("3") + 1).to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        to_yocto("3") + 1,
    );

    assert_eq!(outcome.promise_errors().len(), 0);

    let first_outcome = outcome.promise_results().remove(1).unwrap();
    println!("Outcome {:?}", first_outcome.logs()[0]);

    alice.borrow_runtime_mut().cur_block.block_timestamp = OCTOBER_1_2021 + ONE_DAY * 9;

    let outcome = alice.call(
        marketplace.account_id(),
        "add_bid",
        &json!({
            "nft_contract_id": nft.account_id(),
            "token_id": format!("{}:{}", "1", "1"),
            "ft_token_id": "near",
            "amount": (to_yocto("3") + 2).to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        to_yocto("3") + 2,
    );

    assert_eq!(outcome.promise_errors().len(), 1);
}

#[test]
fn test_add_market_data_dutch_auction() {
    let (marketplace, nft, _, alice, bob, chandra, darmaji, root) = init();

    //owner marketplace and nft-> alice
    //seller -> bob
    //buyer -> chandra
    //treasury -> treasury
    //royalty to 10 different account

    const OCTOBER_1_2021: u64 = 1633046400000000000;
    const ONE_DAY: u64 = 86400000000000;

    create_nft_and_mint_one(&nft, &alice, &bob, &chandra, &darmaji);
    let msg = &json!({
        "market_type":"sale",
        "price": to_yocto("3").to_string(),
        "ft_token_id": "near",
        "is_auction": true,
        "started_at": U64(OCTOBER_1_2021),
        "ended_at": U64(OCTOBER_1_2021 + ONE_DAY*7),
        "end_price": to_yocto("2").to_string(),
    })
    .to_string();

    chandra
        .call(
            marketplace.account_id(),
            "storage_deposit",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            STORAGE_ADD_MARKET_DATA,
        )
        .assert_success();

    let initial_storage_usage = marketplace.account().unwrap().storage_usage;

    let outcome = chandra.call(
        nft.account_id(),
        "nft_approve",
        &json!({
            "token_id": format!("{}:{}", "1", "1"),
            "account_id": marketplace.account_id(),
            "msg": msg,
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        STORAGE_APPROVE,
    );

    outcome.assert_success();
    let storage_price_for_add_market = (marketplace.account().unwrap().storage_usage
        - initial_storage_usage) as u128
        * 10u128.pow(19);
    //println!("{:?}", outcome.promise_results());
    println!(
        "[ADD MARKET DATA DUTCH AUCTION] Gas burnt: {} TeraGas",
        outcome.gas_burnt().0 as f64 / 1e12
    );
    println!(
        "[ADD MARKET DATA DUTCH AUCTION] Storage price : {} yoctoNEAR",
        storage_price_for_add_market
    );

    alice.borrow_runtime_mut().cur_block.block_timestamp = OCTOBER_1_2021 + ONE_DAY * 1;

    let market_data: MarketDataJson = alice
        .view(
            marketplace.account_id(),
            "get_market_data",
            &json!({
                "nft_contract_id": nft.account_id(),
                "token_id": "1:1"
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();

    println!(
        "[DUTCH AUCTION] Price after one day: {}",
        market_data.price.0
    );

    alice.borrow_runtime_mut().cur_block.block_timestamp = OCTOBER_1_2021 + ONE_DAY * 2;

    let market_data: MarketDataJson = alice
        .view(
            marketplace.account_id(),
            "get_market_data",
            &json!({
                "nft_contract_id": nft.account_id(),
                "token_id": "1:1"
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();

    println!(
        "[DUTCH AUCTION] Price after two day: {}",
        market_data.price.0
    );

    //buyer
    let buyer_person = root.create_user(account_o(), to_yocto("100"));
    let outcome = buyer_person.call(
        marketplace.account_id(),
        "buy",
        &json!({
            "nft_contract_id": nft.account_id(),
            "token_id": format!("{}:{}", "1", "1"),
        })
        .to_string()
        .into_bytes(),
        GAS_BUY,
        market_data.price.0,
    );

    outcome.assert_success();
}

#[test]
fn test_50_bid_and_cancel() {
  let (marketplace, nft, _, alice, bob, chandra, darmaji, root) = init();

  create_nft_and_mint_one(&nft, &alice, &bob, &chandra, &darmaji);

  let msg = &json!({
    "market_type": "sale",
    "price": "10",
    "ft_token_id": "near",
    "ended_at": "1655966796000000000",
    "is_auction": true,
  }).to_string(); 

  chandra
        .call(
            marketplace.account_id(),
            "storage_deposit",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            STORAGE_ADD_MARKET_DATA,
        )
        .assert_success();

  chandra.call(
    nft.account_id(), 
    "nft_approve", 
    &json!({
      "token_id": format!("{}:{}", "1", "1"),
      "account_id": marketplace.account_id(),
      "msg": msg
    }).to_string().into_bytes(), 
    DEFAULT_GAS, 
    STORAGE_APPROVE);

  let mut users = vec![];
  let mut bid_amount = 10 * 10u128.pow(24);
  for x in 0..150 {
    let user_id: String = format!("user-{}", x.to_string());
    users.push(root.create_user(AccountId::new_unchecked(user_id), to_yocto("100000")));
    bid_amount = bid_amount + bid_amount / 100 * 5;

    users[x].call(
        marketplace.account_id(),
        "add_bid",
        &json!({
            "nft_contract_id": nft.account_id(),
            "token_id": format!("{}:{}", "1", "1"),
            "ft_token_id": "near",
            "amount": bid_amount.to_string()
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        bid_amount
    ).assert_success();
  }

  chandra.call(
    marketplace.account_id(), 
    "delete_market_data", 
    &json!({
      "nft_contract_id": nft.account_id(),
      "token_id": format!("{}:{}", "1", "1"),
    }).to_string().into_bytes(), 
    DEFAULT_GAS, 
    1
  ).assert_success();

}
