use near_sdk::AccountId;
use near_sdk_sim::{view};

use crate::utils::{init};
mod utils;

#[test]
fn test_new() {
    let (marketplace, nft, treasury, alice, bob) = init();

    let treasury_id: AccountId = view!(marketplace.get_treasury()).unwrap_json();
    assert_eq!(treasury_id, treasury.account_id());
}