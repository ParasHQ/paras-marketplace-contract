use std::collections::HashMap;
use near_sdk::{serde_json::json, AccountId};
use near_sdk_sim::{
    deploy, init_simulator, to_yocto, ContractAccount, UserAccount, STORAGE_AMOUNT,
};
use near_sdk_sim::lazy_static_include::syn::export::str;
use paras_marketplace_contract::ContractContract as MarketplaceContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    NFT_WASM_BYTES => "out/paras_nft_contract.wasm",
    MARKETPLACE_WASM_BYTES => "out/main.wasm",
}

pub const DEFAULT_GAS: u64 = near_sdk_sim::DEFAULT_GAS;
pub const NFT_ID_STR: &str = "nft";

pub const STORAGE_MINT_ESTIMATE: u128 = 11280000000000000000000;
pub const STORAGE_CREATE_SERIES_ESTIMATE: u128 = 23900000000000000000000;

// After calculation
pub const STORAGE_ADD_MARKET_DATA: u128 = 8590000000000000000000;
pub const STORAGE_APPROVE: u128 = 760000000000000000000;
pub const GAS_BUY: u64 = 100 * 10u64.pow(12);

pub fn create_nft_and_mint_one(
    nft: &UserAccount,
    owner: &UserAccount,
    creator: &UserAccount,
    receiver_id_1: &UserAccount,
    receiver_id_2: &UserAccount,
) {

    let royalties: HashMap<AccountId,u32> = HashMap::from([
        ( owner.account_id.clone(), 100u32),
        ( AccountId::new_unchecked("h".repeat(64)) , 100u32),
        ( AccountId::new_unchecked("h2".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("h3".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("h4".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("h5".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("h6".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("h7".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("h8".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("h9".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("i".repeat(64)) , 100u32),
        ( AccountId::new_unchecked("i2".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("i3".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("i4".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("i5".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("i6".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("i7".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("i8".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("i9".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("j".repeat(64)) , 100u32),
        ( AccountId::new_unchecked("j2".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("j3".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("j4".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("j5".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("j6".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("j7".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("j8".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("j9".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("k".repeat(64)) , 100u32),
        ( AccountId::new_unchecked("k2".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("k3".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("k4".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("k5".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("k6".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("k7".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("k8".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("k9".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("l".repeat(64)) , 100u32),
        ( AccountId::new_unchecked("l2".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("l3".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("l4".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("l5".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("l6".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("l7".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("l8".repeat(32)) , 100u32),
        ( AccountId::new_unchecked("l9".repeat(32)) , 100u32)
    ]);
    println!("{}",&json!({
                "token_metadata": {
                    "title": "A".repeat(200),
                    "reference": "A".repeat(59),
                    "media": "A".repeat(59),
                    "copies": 100u64,
                },
                "creator_id": creator.account_id(),
                "price": to_yocto("1").to_string(),
                "royalty": royalties,
            })
        .to_string());
    creator
        .call(
            nft.account_id(),
            "nft_create_series",
            &json!({
                "token_metadata": {
                    "title": "A".repeat(200),
                    "reference": "A".repeat(59),
                    "media": "A".repeat(59),
                    "copies": 100u64,
                },
                "creator_id": creator.account_id(),
                "price": to_yocto("1").to_string(),
                "royalty": royalties,
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            STORAGE_CREATE_SERIES_ESTIMATE * 2, //royalty
        )
        .assert_success();

    receiver_id_1
        .call(
            nft.account_id(),
            "nft_buy",
            &json!({
                "token_series_id": "1",
                "receiver_id": receiver_id_1.account_id(),
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            to_yocto("1") + STORAGE_MINT_ESTIMATE,
        )
        .assert_success();

    receiver_id_2
        .call(
            nft.account_id(),
            "nft_buy",
            &json!({
                "token_series_id": "1",
                "receiver_id": receiver_id_2.account_id(),
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            to_yocto("1") + STORAGE_MINT_ESTIMATE,
        )
        .assert_success();

    receiver_id_2
        .call(
            nft.account_id(),
            "nft_buy",
            &json!({
                "token_series_id": "1",
                "receiver_id": receiver_id_2.account_id(),
            })
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            to_yocto("1") + STORAGE_MINT_ESTIMATE,
        )
        .assert_success();
}

pub fn init() -> (
    ContractAccount<MarketplaceContract>,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
) {
    let root = init_simulator(None);

    let treasury = root.create_user(
        AccountId::new_unchecked("treasury".to_string()),
        to_yocto("100000"),
    );


    root.create_user(account_from(&"g"), to_yocto("100"));
    root.create_user(account_from(&"g2"), to_yocto("100"));
    root.create_user(account_from(&"g3"), to_yocto("100"));
    root.create_user(account_from(&"g4"), to_yocto("100"));
    root.create_user(account_from(&"g5"), to_yocto("100"));
    root.create_user(account_from(&"g6"), to_yocto("100"));
    root.create_user(account_from(&"g7"), to_yocto("100"));
    root.create_user(account_from(&"g8"), to_yocto("100"));
    root.create_user(account_from(&"g9"), to_yocto("100"));

    root.create_user(account_from(&"h"), to_yocto("100"));
    root.create_user(account_from(&"h2"), to_yocto("100"));
    root.create_user(account_from(&"h3"), to_yocto("100"));
    root.create_user(account_from(&"h4"), to_yocto("100"));
    root.create_user(account_from(&"h5"), to_yocto("100"));
    root.create_user(account_from(&"h6"), to_yocto("100"));
    root.create_user(account_from(&"h7"), to_yocto("100"));
    root.create_user(account_from(&"h8"), to_yocto("100"));
    root.create_user(account_from(&"h9"), to_yocto("100"));

    root.create_user(account_from(&"i"), to_yocto("100"));
    root.create_user(account_from(&"i2"), to_yocto("100"));
    root.create_user(account_from(&"i3"), to_yocto("100"));
    root.create_user(account_from(&"i4"), to_yocto("100"));
    root.create_user(account_from(&"i5"), to_yocto("100"));
    root.create_user(account_from(&"i6"), to_yocto("100"));
    root.create_user(account_from(&"i7"), to_yocto("100"));
    root.create_user(account_from(&"i8"), to_yocto("100"));
    root.create_user(account_from(&"i9"), to_yocto("100"));

    root.create_user(account_from(&"j"), to_yocto("100"));
    root.create_user(account_from(&"j2"), to_yocto("100"));
    root.create_user(account_from(&"j3"), to_yocto("100"));
    root.create_user(account_from(&"j4"), to_yocto("100"));
    root.create_user(account_from(&"j5"), to_yocto("100"));
    root.create_user(account_from(&"j6"), to_yocto("100"));
    root.create_user(account_from(&"j7"), to_yocto("100"));
    root.create_user(account_from(&"j8"), to_yocto("100"));
    root.create_user(account_from(&"j9"), to_yocto("100"));

    root.create_user(account_from(&"k"), to_yocto("100"));
    root.create_user(account_from(&"k2"), to_yocto("100"));
    root.create_user(account_from(&"k3"), to_yocto("100"));
    root.create_user(account_from(&"k4"), to_yocto("100"));
    root.create_user(account_from(&"k5"), to_yocto("100"));
    root.create_user(account_from(&"k6"), to_yocto("100"));
    root.create_user(account_from(&"k7"), to_yocto("100"));
    root.create_user(account_from(&"k8"), to_yocto("100"));
    root.create_user(account_from(&"k9"), to_yocto("100"));

    root.create_user(account_from(&"l"), to_yocto("100"));
    root.create_user(account_from(&"l2"), to_yocto("100"));
    root.create_user(account_from(&"l3"), to_yocto("100"));
    root.create_user(account_from(&"l4"), to_yocto("100"));
    root.create_user(account_from(&"l5"), to_yocto("100"));
    root.create_user(account_from(&"l6"), to_yocto("100"));
    root.create_user(account_from(&"l7"), to_yocto("100"));
    root.create_user(account_from(&"l8"), to_yocto("100"));
    root.create_user(account_from(&"l9"), to_yocto("100"));

    root.create_user(account_from(&"m"), to_yocto("100"));
    root.create_user(account_from(&"m2"), to_yocto("100"));
    root.create_user(account_from(&"m3"), to_yocto("100"));
    root.create_user(account_from(&"m4"), to_yocto("100"));
    root.create_user(account_from(&"m5"), to_yocto("100"));
    root.create_user(account_from(&"m6"), to_yocto("100"));
    root.create_user(account_from(&"m7"), to_yocto("100"));
    root.create_user(account_from(&"m8"), to_yocto("100"));
    root.create_user(account_from(&"m9"), to_yocto("100"));

    let alice = root.create_user(account_from(&"x"), to_yocto("100"));

    let bob = root.create_user(account_from(&"y"), to_yocto("100"));

    let chandra = root.create_user(account_from(&"z"), to_yocto("100"));

    let darmaji = root.create_user(account_from(&"n"), to_yocto("100"));
    let nft_account_id = AccountId::new_unchecked(NFT_ID_STR.to_string());
    let nft_contract = root.deploy(&NFT_WASM_BYTES, nft_account_id.clone(), STORAGE_AMOUNT);

    nft_contract.call(
        nft_account_id,
        "new_default_meta",
        &json!({
            "owner_id": alice.account_id(),
            "treasury_id": treasury.account_id(),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    let marketplace_contract = deploy!(
        contract: MarketplaceContract,
        contract_id: &AccountId::new_unchecked("mk".repeat(32)),
        bytes: &MARKETPLACE_WASM_BYTES,
        signer_account: root,
        init_method: new(
            alice.account_id(),
            treasury.account_id(),
            None,
            Some(vec!(nft_contract.account_id())),
            Some(vec!(nft_contract.account_id())),
            500
        )
    );

    (
        marketplace_contract,
        nft_contract,
        treasury,
        alice,
        bob,
        chandra,
        darmaji,
        root,
    )
}

pub fn account_o() -> AccountId {
    account_from("o")
}

pub fn account_from(s: &str) -> AccountId {
    if s.len()==2{
        AccountId::new_unchecked(s.repeat(32).to_string())
    }else {
        AccountId::new_unchecked(s.repeat(64).to_string())

    }
}
