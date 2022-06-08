use crate::*;
/// approval callbacks from NFT Contracts
const DELIMETER: &str = "||";
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MarketArgs {
    pub market_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<U128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ft_token_id: Option<AccountId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<AccountId>, // offer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_price: Option<U128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<U64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<U64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auction: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_nft_contract_id: Option<AccountId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_token_id: Option<TokenId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_token_series_id: Option<TokenSeriesId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_nft_contract_id: Option<AccountId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_token_id: Option<TokenId>,
}

trait NonFungibleTokenApprovalsReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}

#[near_bindgen]
impl NonFungibleTokenApprovalsReceiver for Contract {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    ) {
        // enforce cross contract call and owner_id is signer

        let nft_contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();
        assert_ne!(
            env::current_account_id(), nft_contract_id,
            "Paras: nft_on_approve should only be called via cross-contract call"
        );
        assert_eq!(owner_id, signer_id, "Paras: owner_id should be signer_id");

        assert!(
            self.approved_nft_contract_ids.contains(&nft_contract_id),
            "Paras: nft_contract_id is not approved"
        );

        let MarketArgs {
            market_type,
            price,
            ft_token_id,
            buyer_id,
            started_at,
            ended_at,
            end_price,
            is_auction,
            seller_nft_contract_id,
            seller_token_id,
            seller_token_series_id,
            buyer_nft_contract_id,
            buyer_token_id
        } = near_sdk::serde_json::from_str(&msg).expect("Not valid MarketArgs");

        if market_type == "sale" {
            assert!(price.is_some(), "Paras: price not specified");

            self.internal_delete_market_data(&nft_contract_id, &token_id);

            let storage_amount = self.storage_minimum_balance().0;
            let owner_paid_storage = self.storage_deposits.get(&signer_id).unwrap_or(0);
            let signer_storage_required =
                (self.get_supply_by_owner_id(signer_id).0 + 1) as u128 * storage_amount;

            assert!(
                owner_paid_storage >= signer_storage_required,
                "Insufficient storage paid: {}, for {} sales at {} rate of per sale",
                owner_paid_storage,
                signer_storage_required / storage_amount,
                storage_amount,
            );

            let ft_token_id_res = ft_token_id.unwrap_or(near_account());

            if self.approved_ft_token_ids.contains(&ft_token_id_res) != true {
                env::panic_str(&"Paras: ft_token_id not approved");
            }

            self.internal_add_market_data(
                owner_id,
                approval_id,
                nft_contract_id,
                token_id,
                ft_token_id_res,
                price.unwrap(),
                started_at,
                ended_at,
                end_price,
                is_auction,
            );
        } else if market_type == "accept_offer" {
            assert!(buyer_id.is_some(), "Paras: Account id is not specified");
            assert!(price.is_some(), "Paras: Price is not specified (for check)");

            self.internal_accept_offer(
                nft_contract_id,
                buyer_id.unwrap(),
                token_id,
                owner_id,
                approval_id,
                price.unwrap().0,
            );
        } else if market_type == "accept_offer_paras_series" {
            assert!(buyer_id.is_some(), "Paras: Account id is not specified");
            assert!(
                self.paras_nft_contracts.contains(&nft_contract_id),
                "Paras: accepting offer series for Paras NFT only"
            );
            assert!(price.is_some(), "Paras: Price is not specified (for check)");

            self.internal_accept_offer_series(
                nft_contract_id,
                buyer_id.unwrap(),
                token_id,
                owner_id,
                approval_id,
                price.unwrap().0,
            );
        } else if market_type == "add_trade" {
            // old market data
            let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
            if let Some(mut market_data) = self.market.get(&contract_and_token_id) {
                market_data.approval_id = approval_id;
                self.market.insert(&contract_and_token_id, &market_data);
            }
            //replace old data approval id
            let buyer_contract_account_id_token_id = make_triple(&nft_contract_id,
                            &owner_id,
                            &token_id);
            if let Some(mut old_trade) = self.trades.get(&buyer_contract_account_id_token_id){
                self.trades.remove(&buyer_contract_account_id_token_id);
                let new_trade_list=TradeList {
                    approval_id,
                    trade_data: old_trade.trade_data,
                };
                // old_trade.approval_id = approval_id
                // self.trades.insert(&buyer_contract_account_id_token_id,&old_trade);
                self.trades.insert(&buyer_contract_account_id_token_id,&new_trade_list);
            }

            let storage_amount = self.storage_minimum_balance().0;
            let owner_paid_storage = self.storage_deposits.get(&signer_id).unwrap_or(0);
            let signer_storage_required =
                (self.get_supply_by_owner_id(signer_id).0 + 1) as u128 * storage_amount;

            if owner_paid_storage <= signer_storage_required {
                let notif=format!("Insufficient storage paid: {}, for {} sales at {} rate of per sale",
                                  owner_paid_storage,
                                  signer_storage_required / storage_amount,
                                  storage_amount
                );
                env::log_str(&notif);
                return;
            }

            self.add_trade(
                seller_nft_contract_id.unwrap(),
                seller_token_id,
                seller_token_series_id,
                nft_contract_id,
                owner_id,
                Some(token_id),
                approval_id,
            );
        } else if market_type == "accept_trade" {
            assert!(buyer_id.is_some(), "Paras: Account id is not specified");
            assert!(buyer_nft_contract_id.is_some(), "Paras: Buyer NFT contract id is not specified");
            assert!(buyer_token_id.is_some(), "Paras: Buyer token id is not specified");
            let wbuyer_nft_contract_id= buyer_nft_contract_id.unwrap();
            let wbuyer_id= buyer_id.unwrap();
            let wbuyer_token_id= buyer_token_id.unwrap();

            let buyer_contract_account_id_token_id =
                make_triple(&wbuyer_nft_contract_id,
                            &wbuyer_id,
                            &wbuyer_token_id);
            let contract_account_id_token_id = make_triple(&nft_contract_id,
                                                           &wbuyer_id,
                                                           &token_id);

            if self
                .trades
                .get(&buyer_contract_account_id_token_id).is_some(){
                env::log_str("Paras: Trade list does not exist");
                return;
            }

            if self
                .trades
                .get(&buyer_contract_account_id_token_id).unwrap()
                .trade_data
                .get(&contract_account_id_token_id)
                .is_some(){
                env::log_str("Paras: Trade data does not exist");
                return;
            }

            self.internal_accept_trade(
                nft_contract_id,
                wbuyer_id,
                token_id,
                owner_id,
                approval_id,
                wbuyer_nft_contract_id,
                wbuyer_token_id
            );
        } else if market_type == "accept_trade_paras_series" {
            assert!(buyer_id.is_some(), "Paras: Account id is not specified");
            assert!(
                self.paras_nft_contracts.contains(&nft_contract_id),
                "Paras: accepting offer series for Paras NFT only"
            );
            assert!(buyer_nft_contract_id.is_some(), "Paras: Buyer NFT contract id is not specified");
            assert!(buyer_token_id.is_some(), "Paras: Buyer token id is not specified");
            let mut token_id_iter = token_id.split(":");
            let token_series_id: String = token_id_iter.next().unwrap().parse().unwrap();

            let wbuyer_nft_contract_id= buyer_nft_contract_id.unwrap();
            let wbuyer_id= buyer_id.unwrap();
            let wbuyer_token_id= buyer_token_id.unwrap();

            let buyer_contract_account_id_token_id =
                make_triple(&wbuyer_nft_contract_id,
                            &wbuyer_id,
                            &wbuyer_token_id);
            let contract_account_id_token_id = make_triple(&nft_contract_id,
                                                           &wbuyer_id,
                                                           &token_series_id);

            if self
                .trades
                .get(&buyer_contract_account_id_token_id).is_some(){
                env::log_str("Paras: Trade list does not exist");
                return;
            }

            if self
                .trades
                .get(&buyer_contract_account_id_token_id).unwrap()
                .trade_data
                .get(&contract_account_id_token_id)
                .is_some(){
                env::log_str("Paras: Trade data does not exist");
                return;
            }

            self.internal_accept_trade_series(
                nft_contract_id,
                wbuyer_id,
                token_id,
                owner_id,
                approval_id,
                wbuyer_nft_contract_id,
                wbuyer_token_id
            );
        }
    }
}

fn make_triple(nft_contract_id: &AccountId, buyer_id: &AccountId, token: &str) -> String {
    format!(
        "{}{}{}{}{}",
        nft_contract_id, DELIMETER, buyer_id, DELIMETER, token
    )
}