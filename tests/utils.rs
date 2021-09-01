use paras_marketplace_contract::ContractContract as MarketplaceContract;
use near_sdk::serde_json::json;
use near_sdk_sim::{
    deploy, init_simulator, to_yocto, ContractAccount, UserAccount, DEFAULT_GAS, STORAGE_AMOUNT,
};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    NFT_WASM_BYTES => "out/paras_nft_contract.wasm",
    MARKETPLACE_WASM_BYTES => "out/main.wasm",
}

pub const NFT_ID: &str = "nft";
pub const MARKETPLACE_ID: &str = "marketplace";

pub fn init() -> (ContractAccount<MarketplaceContract>, UserAccount, UserAccount, UserAccount, UserAccount) {
    let root = init_simulator(None);

    let treasury = root.create_user(
        "treasury".to_string(),
        to_yocto("100")
    );

    let alice = root.create_user(
        "alice".to_string(),
        to_yocto("100")
    );

    let bob = root.create_user(
        "bob".to_string(),
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

    (marketplace_contract, nft_contract, treasury, alice, bob)
}