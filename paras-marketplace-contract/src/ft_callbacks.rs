use near_sdk::PromiseOrValue;
use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PurchaseArgs {
    pub nft_contract_id: AccountId,
    pub token_id: TokenId,
}

trait FungibleTokenReceiver {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128> {
        let PurchaseArgs {
           nft_contract_id,
           token_id
        } = near_sdk::serde_json::from_str(&msg).expect("Invalid PurchaseArgs");

        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);

        let market_data: MarketData = self.internal_get_market_data(contract_and_token_id).expect("Paras: Market data does not exist");

        let buyer_id = env::signer_account_id();

        assert_ne!(
            buyer_id, market_data.owner_id,
            "Paras: Cannot buy your own sale"
        );

        let ft_token_id = env::predecessor_account_id();

        assert_eq!(
            market_data.ft_token_id,
            ft_token_id,
            "Paras: Not for sale for that FT"
        );

        let mut price = market_data.price;

        if market_data.is_auction.is_some() && market_data.end_price.is_some() {
            let current_time = env::block_timestamp();
            let end_price = market_data.end_price.unwrap();
            let ended_at = market_data.ended_at.unwrap();
            let started_at = market_data.started_at.unwrap();

            assert!(
                current_time >= started_at,
                "Paras: Auction has not started yet"
            );

            if current_time > ended_at {
                price = end_price;
            } else {
                let time_since_start = current_time - started_at;
                let duration = ended_at - started_at;
                price = price - ((price - end_price) / duration as u128) * time_since_start as u128;
            }
        } else if let Some(auction) = market_data.is_auction {
            assert_eq!(auction, false, "Paras: the NFT is on auction");
        }

        assert!(
            env::attached_deposit() >= price,
            "Paras: Attached deposit is less than price {}",
            price
        );

        near_sdk::PromiseOrValue::Promise(self.internal_process_purchase(nft_contract_id.into(), token_id, buyer_id, price))
    }
}

