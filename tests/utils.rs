use paras_marketplace_contract::ContractContract as MarketplaceContract;
use near_sdk::serde_json::json;
use near_sdk::json_types::{U128};
use near_sdk_sim::{
    deploy, init_simulator, to_yocto, ContractAccount, UserAccount, DEFAULT_GAS, STORAGE_AMOUNT,
};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    NFT_WASM_BYTES => "out/paras_nft_contract.wasm",
    MARKETPLACE_WASM_BYTES => "out/main.wasm",
}

pub const NFT_ID: &str = "nft";
pub const MARKETPLACE_ID: &str = "marketplace";

pub const STORAGE_MINT_ESTIMATE: u128 = 11280000000000000000000;
pub const STORAGE_CREATE_SERIES_ESTIMATE: u128 = 8540000000000000000000;

// After calculation
pub const STORAGE_ADD_MARKET_DATA: u128 = 4020000000000000000000;

pub fn create_nft_and_mint_one(
    nft: &UserAccount, 
    owner: &UserAccount, 
    creator: &UserAccount,
    receiver_id: &UserAccount,
) {
    owner.call(
        nft.account_id(),
        "nft_create_series",
        &json!({
            "token_series_id": u128::MAX.to_string(),
            "token_metadata": {
                "title": "A".repeat(200),
                "reference": "A".repeat(59),
                "media": "A".repeat(59),
                "copies": 100u64,
            },
            "creator_id": creator.valid_account_id(), 
            "price": to_yocto("1").to_string(),
            "royalty": {
                owner.account_id.clone(): 1000u32,
                "g".repeat(64): 1000u32,
                "h".repeat(64): 1000u32,
                "i".repeat(64): 1000u32,
                "j".repeat(64): 1000u32,
                "k".repeat(64): 1000u32,
                "l".repeat(64): 1000u32,
                "m".repeat(64): 500u32,
                "n".repeat(64): 500u32,
            },
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        STORAGE_CREATE_SERIES_ESTIMATE*2, //royalty 
    ).assert_success();


    receiver_id.call(
        nft.account_id(),
        "nft_buy",
        &json!({
            "token_series_id": u128::MAX.to_string(),
            "receiver_id": receiver_id.valid_account_id(),
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        to_yocto("1") + STORAGE_MINT_ESTIMATE
    ).assert_success();
}
pub fn init() -> (
    ContractAccount<MarketplaceContract>, 
    UserAccount, 
    UserAccount, 
    UserAccount, 
    UserAccount,
    UserAccount,
    UserAccount
) {
    let root = init_simulator(None);

    let treasury = root.create_user(
        "treasury".to_string(),
        to_yocto("100")
    );

    root.create_user(
        "g".repeat(64),
        to_yocto("100")
    );

    root.create_user(
        "h".repeat(64),
        to_yocto("100")
    );

    root.create_user(
        "i".repeat(64),
        to_yocto("100")
    );

    root.create_user(
        "j".repeat(64),
        to_yocto("100")
    );

    root.create_user(
        "k".repeat(64),
        to_yocto("100")
    );

    root.create_user(
        "l".repeat(64),
        to_yocto("100")
    );

    root.create_user(
        "m".repeat(64),
        to_yocto("100")
    );

    root.create_user(
        "n".repeat(64),
        to_yocto("100")
    );

    let alice = root.create_user(
        "x".repeat(64),
        to_yocto("100")
    );

    let bob = root.create_user(
        "y".repeat(64),
        to_yocto("100")
    );

    let chandra = root.create_user(
        "z".repeat(64),
        to_yocto("100")
    );

    let nft_contract = root.deploy(
        &NFT_WASM_BYTES,
        NFT_ID.to_string(),
        STORAGE_AMOUNT
    );

    nft_contract.call(
        NFT_ID.into(),
        "new_default_meta",
        &json!({
            "owner_id": alice.valid_account_id(),
            "treasury_id": treasury.valid_account_id(),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    let marketplace_contract = deploy!(
        contract: MarketplaceContract,
        contract_id: MARKETPLACE_ID,
        bytes: &MARKETPLACE_WASM_BYTES,
        signer_account: root,
        init_method: new(
            alice.valid_account_id(),
            treasury.valid_account_id(),
            None,
            Some(vec!(nft_contract.valid_account_id()))
        )
    );

    (marketplace_contract, nft_contract, treasury, alice, bob, chandra, root)
}