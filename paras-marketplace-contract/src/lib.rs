use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet, LookupMap};
use near_sdk::json_types::{U128, U64, ValidAccountId};
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, CryptoHash,
    PanicOnDefault, serde_json::json, assert_one_yocto, ext_contract, Gas, Balance, Promise
};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::promise_result_as_success;
use std::collections::HashMap;

use crate::external::*;

mod external;
mod nft_callbacks;

const GAS_FOR_NFT_TRANSFER: Gas = 20_000_000_000_000;
const BASE_GAS: Gas = 5_000_000_000_000;
const GAS_FOR_ROYALTIES: Gas = BASE_GAS * 10;
const NO_DEPOSIT: Balance = 0;
const TREASURY_FEE: u128 = 500; // 500 /10_000 = 0.05 

pub const STORAGE_ADD_MARKET_DATA: u128 = 8590000000000000000000;

near_sdk::setup_alloc!();

pub type Payout = HashMap<AccountId, U128>;
pub type ContractAndTokenId = String;
pub type ContractAccountIdTokenId = String;
pub type TokenId = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid {
    pub bidder_id: AccountId,
    pub price: U128,
}

pub type Bids = Vec<Bid>; 

const DELIMETER: &str = "||";

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MarketData {
    pub owner_id: AccountId,
    pub approval_id: u64,
    pub nft_contract_id: String,
    pub token_id: TokenId,
    pub ft_token_id: AccountId, // "near" for NEAR token
    pub price: u128, // if auction, price becomes starting price
    pub bids: Option<Bids>,
    pub started_at: Option<u64>,
    pub ended_at: Option<u64>,
    pub end_price: Option<u128>, // dutch auction
    pub accept_nft_contract_id: Option<String>,
    pub accept_token_id: Option<String>,
    pub is_auction: Option<bool>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OfferData {
    pub buyer_id: AccountId,
    pub nft_contract_id: String,
    pub token_id: Option<TokenId>,
    pub token_series_id: Option<TokenId>,
    pub ft_token_id: AccountId, // "near" for NEAR token
    pub price: u128,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OfferDataJson {
    buyer_id: AccountId,
    nft_contract_id: String,
    token_id: Option<TokenId>,
    token_series_id: Option<TokenId>,
    ft_token_id: AccountId, // "near" for NEAR token
    price: U128,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MarketDataJson {
    owner_id: AccountId,
    approval_id: U64,
    nft_contract_id: String,
    token_id: TokenId,
    ft_token_id: AccountId, // "near" for NEAR token
    price: U128,
    bids: Option<Bids>,
    started_at: Option<U64>,
    ended_at: Option<U64>,
    end_price: Option<U128>, // dutch auction
    is_auction: Option<bool>,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ContractV1 {
    pub owner_id: AccountId,
    pub treasury_id: AccountId,
    pub market: UnorderedMap<ContractAndTokenId, MarketData>,
    pub approved_ft_token_ids: UnorderedSet<AccountId>,
    pub approved_nft_contract_ids: UnorderedSet<AccountId>,
    pub storage_deposits: LookupMap<AccountId, Balance>,
    pub by_owner_id: LookupMap<AccountId, UnorderedSet<TokenId>>
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub treasury_id: AccountId,
    pub market: UnorderedMap<ContractAndTokenId, MarketData>,
    pub approved_ft_token_ids: UnorderedSet<AccountId>,
    pub approved_nft_contract_ids: UnorderedSet<AccountId>,
    pub storage_deposits: LookupMap<AccountId, Balance>,
    pub by_owner_id: LookupMap<AccountId, UnorderedSet<TokenId>>,
    pub offers: UnorderedMap<ContractAccountIdTokenId, OfferData>,
    pub paras_nft_contracts: UnorderedSet<AccountId>,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Market,
    FTTokenIds,
    NFTContractIds,
    StorageDeposits,
    ByOwnerId,
    ByOwnerIdInner { account_id_hash: CryptoHash },
    Offers,
    ParasNFTContractIds,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: ValidAccountId, 
        treasury_id: ValidAccountId,
        approved_ft_token_ids: Option<Vec<ValidAccountId>>,
        approved_nft_contract_ids: Option<Vec<ValidAccountId>>,
        paras_nft_contracts: Option<Vec<ValidAccountId>>,
    ) -> Self {
        let mut this = Self {
            owner_id: owner_id.into(),
            treasury_id: treasury_id.into(),
            market: UnorderedMap::new(StorageKey::Market),
            approved_ft_token_ids: UnorderedSet::new(StorageKey::FTTokenIds),
            approved_nft_contract_ids: UnorderedSet::new(StorageKey::NFTContractIds),
            storage_deposits: LookupMap::new(StorageKey::StorageDeposits),
            by_owner_id: LookupMap::new(StorageKey::ByOwnerId),
            offers: UnorderedMap::new(StorageKey::Offers),
            paras_nft_contracts: UnorderedSet::new(StorageKey::ParasNFTContractIds),
        };

        this.approved_ft_token_ids.insert(&"near".to_string());
        
        if let Some(approved_ft_token_ids) = approved_ft_token_ids {
            for approved_ft_token_id in approved_ft_token_ids {
                this.approved_ft_token_ids.insert(approved_ft_token_id.as_ref());
            }
        }

        if let Some(approved_nft_contract_ids) = approved_nft_contract_ids{
            for approved_nft_contract_id in approved_nft_contract_ids {
                this.approved_nft_contract_ids.insert(approved_nft_contract_id.as_ref());
            }
        }

        if let Some(paras_nft_contracts) = paras_nft_contracts {
            for paras_nft_contract in paras_nft_contracts {
                this.paras_nft_contracts.insert(paras_nft_contract.as_ref());
            }
        }

        this
    }

    #[init(ignore_state)]
    pub fn migrate(paras_nft_contracts: Option<Vec<ValidAccountId>>) -> Self {
        let prev: ContractV1 = env::state_read().expect("ERR_NOT_INITIALIZED");
        assert_eq!(
            env::predecessor_account_id(),
            prev.owner_id,
            "Paras: Only owner"
        );

        let mut this = Contract {
            owner_id: prev.owner_id,
            treasury_id: prev.treasury_id,
            market: prev.market,
            approved_ft_token_ids: prev.approved_ft_token_ids,
            approved_nft_contract_ids: prev.approved_nft_contract_ids,
            storage_deposits: prev.storage_deposits,
            by_owner_id: prev.by_owner_id,
            offers: UnorderedMap::new(StorageKey::Offers),
            paras_nft_contracts: UnorderedSet::new(StorageKey::ParasNFTContractIds), 
        };

        if let Some(paras_nft_contracts) = paras_nft_contracts {
            for paras_nft_contract in paras_nft_contracts {
                this.paras_nft_contracts.insert(paras_nft_contract.as_ref());
            }
        }

        this
    }

    // Changing treasury & ownership

    #[payable]
    pub fn set_treasury(&mut self, treasury_id: ValidAccountId) {
        assert_one_yocto();
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Paras: Owner only"
        );
        self.treasury_id = treasury_id.to_string();
    }

    #[payable]
    pub fn transfer_ownership(&mut self, owner_id: ValidAccountId) {
        assert_one_yocto();
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Paras: Owner only"
        );
        self.owner_id = owner_id.to_string();
    }

    // Approved contracts
    #[payable]
    pub fn add_approved_nft_contract_ids(&mut self, nft_contract_ids: Vec<ValidAccountId>) {
        assert_one_yocto();
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Paras: Owner only"
        );
        for nft_contract_id in nft_contract_ids {
            self.approved_nft_contract_ids.insert(nft_contract_id.as_ref());
        }
    }

    #[payable]
    pub fn add_approved_ft_token_ids(
        &mut self,
        ft_token_ids: Vec<ValidAccountId>
    ) {
        assert_one_yocto();
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Paras: Owner only"
        );
        for ft_token_id in ft_token_ids {
            self.approved_ft_token_ids.insert(ft_token_id.as_ref());
        }
    }

    // Buy & Payment

    #[payable]
    pub fn buy(
        &mut self, 
        nft_contract_id: ValidAccountId,
        token_id: TokenId,
    ) {
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
        let market_data = self.market.get(&contract_and_token_id).expect("Paras: Token id does not exist");
        let buyer_id = env::predecessor_account_id();

        assert_ne!(
            buyer_id,
            market_data.owner_id,
            "Paras: Cannot buy your own sale"
        );

        // only NEAR supported for now
        assert_eq!(
            market_data.ft_token_id,
            "near",
            "Paras: NEAR support only"
        );

        let mut price = market_data.price;

        if market_data.is_auction.is_some() && market_data.end_price.is_some() {
            let current_time = env::block_timestamp();
            let end_price = market_data.end_price.unwrap();
            let ended_at = market_data.ended_at.unwrap();
            let started_at = market_data.started_at.unwrap();

            assert!(current_time >= started_at, "Paras: Auction has not started yet");

            if current_time > ended_at {
                price = end_price;
            } else {
                let time_since_start = current_time - started_at;
                let duration = ended_at - started_at;
                price = price - ((price - end_price) / duration as u128) * time_since_start as u128;
            }

        } else if let Some(auction) = market_data.is_auction {
            assert_eq!(
                auction,
                false,
                "Paras: the NFT is on auction"
            );
        }

        assert!(
            env::attached_deposit() >= price,
            "Paras: Attached deposit is less than price {}", 
            price
        );

        self.internal_process_purchase(
            nft_contract_id.into(),
            token_id,
            buyer_id,
            price,
        );
    }

    fn internal_process_purchase(
        &mut self,
        nft_contract_id: AccountId,
        token_id: TokenId,
        buyer_id: AccountId,
        price: u128,
    ) -> Promise {

        let market_data = self.internal_delete_market_data(&nft_contract_id, &token_id).expect("Paras: Sale does not exist");

        ext_contract::nft_transfer_payout(
            buyer_id.clone(),
            token_id,
            Some(market_data.approval_id),
            Some(U128::from(price)),
            Some(10u32), // max length payout
            &nft_contract_id,
            1,
            GAS_FOR_NFT_TRANSFER,
        )
        .then(ext_self::resolve_purchase(
            buyer_id,
            market_data,
            U128::from(price),
            &env::current_account_id(),
            NO_DEPOSIT,
            GAS_FOR_ROYALTIES,
        ))
    }

    #[private]
    pub fn resolve_purchase(
        &mut self,
        buyer_id: AccountId,
        market_data: MarketData,
        price: U128
    ) -> U128 {
        let payout_option = promise_result_as_success().and_then(|value| {
            // None means a bad payout from bad NFT contract
            near_sdk::serde_json::from_slice::<Payout>(&value)
                .ok()
                .and_then(|payout| {
                    let mut remainder = price.0;
                    for &value in payout.values() {
                        remainder = remainder.checked_sub(value.0)?;
                    }
                    if remainder == 0 || remainder == 1 {
                        Some(payout)
                    } else {
                        None
                    }
                })
        });
        let payout = if let Some(payout_option) = payout_option {
            payout_option
        } else {
            if market_data.ft_token_id == "near" {
                Promise::new(buyer_id.clone()).transfer(u128::from(market_data.price));
            }
            // leave function and return all FTs in ft_resolve_transfer
            env::log(
                json!({
                    "type": "resolve_purchase_fail",
                    "params": {
                        "owner_id": market_data.owner_id,
                        "nft_contract_id": market_data.nft_contract_id,
                        "token_id": market_data.token_id,
                        "ft_token_id": market_data.ft_token_id,
                        "price": price,
                        "buyer_id": buyer_id,
                    }
                })
                .to_string()
                .as_bytes(),
            );
            return price;
        };

        // Payout (transfer to royalties and seller)
        if market_data.ft_token_id == "near" {
            // 5% fee for treasury
            let treasury_fee = price.0 * TREASURY_FEE / 10_000u128;

            for (receiver_id, amount) in payout {
                if receiver_id == market_data.owner_id {
                    Promise::new(receiver_id).transfer(amount.0 - treasury_fee);
                    Promise::new(self.treasury_id.clone()).transfer(treasury_fee);
                } else {
                    Promise::new(receiver_id).transfer(amount.0);
                }
                
            }
            env::log(
                json!({
                    "type": "resolve_purchase",
                    "params": {
                        "owner_id": market_data.owner_id,
                        "nft_contract_id": market_data.nft_contract_id,
                        "token_id": market_data.token_id,
                        "ft_token_id": market_data.ft_token_id,
                        "price": price,
                        "buyer_id": buyer_id,
                    }
                })
                .to_string()
                .as_bytes(),
            );

            return price;
        } else {
            U128(0)
        }
    }

    // Offer

    fn internal_add_offer(
        &mut self,
        nft_contract_id: AccountId,
        token_id: Option<TokenId>,
        token_series_id: Option<TokenId>,
        ft_token_id: AccountId,
        price: U128,
        account_id: AccountId,
    ) {
        let token = if token_id.is_some() {
            token_id.as_ref().unwrap().to_string()
        } else {
            token_series_id.as_ref().unwrap().to_string()
        };

        let contract_account_id_token_id = format!("{}{}{}{}{}", nft_contract_id, DELIMETER, account_id, DELIMETER, token);
        self.offers.insert(
            &contract_account_id_token_id,
            &OfferData {
                buyer_id: account_id.clone().into(),
                nft_contract_id: nft_contract_id.into(),
                token_id: token_id,
                token_series_id: token_series_id,
                ft_token_id: ft_token_id.into(),
                price: price.into(),
            },
        );

        let mut token_ids = self.by_owner_id.get(&account_id).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::ByOwnerIdInner {
                    account_id_hash: hash_account_id(&account_id),
                }
                    .try_to_vec()
                    .unwrap()
            )
        });
        token_ids.insert(&contract_account_id_token_id);
        self.by_owner_id.insert(&account_id, &token_ids);
    }

    #[payable]
    pub fn add_offer(
        &mut self,
        nft_contract_id: ValidAccountId,
        token_id: Option<TokenId>,
        token_series_id: Option<String>,
        ft_token_id: AccountId,
        price: U128
    ) {
        let token = if token_id.is_some() {
            token_id.as_ref().unwrap().to_string()
        } else {
            assert!(self.paras_nft_contracts.contains(&nft_contract_id.to_string()), "Paras: offer series for Paras NFT only");
            token_series_id.as_ref().unwrap().to_string()
        };

        assert_eq!(
            env::attached_deposit(),
            price.0,
            "Paras: Attached deposit != price"
        );

        assert_eq!(
            ft_token_id.to_string(),
            "near",
            "Paras: Only NEAR is supported"
        );

        let account_id = env::predecessor_account_id();
        let offer_data = self.internal_delete_offer(
            nft_contract_id.clone().into(),
            account_id.clone(),
            token.clone()
        );

        if offer_data.is_some() {
            Promise::new(account_id.clone()).transfer(offer_data.unwrap().price);
        }

        let storage_amount = self.storage_minimum_balance().0;
        let owner_paid_storage = self.storage_deposits.get(&account_id).unwrap_or(0);
        let signer_storage_required =
            (self.get_supply_by_owner_id(account_id.clone()).0 + 1) as u128 * storage_amount;

        assert!(
            owner_paid_storage >= signer_storage_required,
            "Insufficient storage paid: {}, for {} offer at {} rate of per offer",
            owner_paid_storage, signer_storage_required / storage_amount, storage_amount,
        );

        self.internal_add_offer(
            nft_contract_id.clone().into(),
            token_id.clone(),
            token_series_id.clone(),
            ft_token_id.clone(),
            price,
            account_id.clone()
        );

        env::log(
            json!({
                "type": "add_offer",
                "params": {
                    "buyer_id": account_id,
                    "nft_contract_id": nft_contract_id,
                    "token_id": token_id,
                    "token_series_id": token_series_id,
                    "ft_token_id": ft_token_id,
                    "price": price,
                }
            })
            .to_string()
            .as_bytes(),
        );
    }

    fn internal_delete_offer(
        &mut self,
        nft_contract_id: AccountId,
        account_id: AccountId,
        token_id: TokenId,
    ) -> Option<OfferData> {
        let contract_account_id_token_id = format!("{}{}{}{}{}", nft_contract_id, DELIMETER, account_id, DELIMETER, token_id);
        let offer_data = self.offers.remove(&contract_account_id_token_id);

        match offer_data {
            Some(offer) => {
                let mut by_owner_id = self.by_owner_id.get(&offer.buyer_id).expect("Paras: no market data by account_id");
                by_owner_id.remove(&contract_account_id_token_id);
                if by_owner_id.is_empty() {
                    self.by_owner_id.remove(&offer.buyer_id);
                } else {
                    self.by_owner_id.insert(&offer.buyer_id, &by_owner_id);
                }
                return Some(offer);
            },
            None => return None,
        };
    }

    pub fn delete_offer(
        &mut self,
        nft_contract_id: ValidAccountId,
        token_id: Option<TokenId>,
        token_series_id: Option<String>
    ) {
        let token = if token_id.is_some() {
            token_id.as_ref().unwrap().to_string()
        } else {
            token_series_id.as_ref().unwrap().to_string()
        };

        let account_id = env::predecessor_account_id();
        let contract_account_id_token_id = format!("{}{}{}{}{}", nft_contract_id, DELIMETER, account_id, DELIMETER, token);

        let offer_data= self.offers.get(&contract_account_id_token_id).expect("Paras: Offer does not exist");

        if token_id.is_some() {
            assert_eq!(offer_data.token_id.unwrap(), token)
        } else {
            assert_eq!(offer_data.token_series_id.unwrap(), token)
        }

        assert_eq!(offer_data.buyer_id, account_id, "Paras: Caller not offer's buyer");

        self.internal_delete_offer(
            nft_contract_id.clone().into(),
            account_id.clone(),
            token.clone(),
        ).expect("Paras: Offer not found");

        Promise::new(offer_data.buyer_id).transfer(offer_data.price);

        env::log(
            json!({
                "type": "delete_offer",
                "params": {
                    "nft_contract_id": nft_contract_id,
                    "account_id": account_id,
                    "token_id": token_id,
                    "token_series_id": token_series_id,
                }
            })
            .to_string()
            .as_bytes(),
        );
    }

    pub fn get_offer(
        &self,
        nft_contract_id: ValidAccountId,
        account_id: ValidAccountId,
        token_id: Option<TokenId>,
        token_series_id: Option<String>
    ) -> OfferDataJson {
        let token = if token_id.is_some() {
            token_id.as_ref().unwrap()
        } else {
            token_series_id.as_ref().unwrap()
        };

        let contract_account_id_token_id = format!("{}{}{}{}{}", nft_contract_id, DELIMETER, account_id, DELIMETER, token);
        let offer_data = self.offers.get(&contract_account_id_token_id).expect("Paras: Offer does not exist");

        if token_id.is_some() {
            assert_eq!(offer_data.token_id.as_ref().unwrap(), token);
        } else {
            assert_eq!(offer_data.token_series_id.as_ref().unwrap(), token);
        }


        OfferDataJson {
            buyer_id: offer_data.buyer_id,
            nft_contract_id: offer_data.nft_contract_id,
            token_id: offer_data.token_id,
            token_series_id: offer_data.token_series_id,
            ft_token_id: offer_data.ft_token_id,
            price: U128(offer_data.price)
        }
    }

    fn internal_accept_offer(
        &mut self,
        nft_contract_id: AccountId,
        account_id: AccountId,
        token_id: TokenId,
        seller_id: AccountId,
        approval_id: u64,
    ) -> Promise {
        let offer_data = self.internal_delete_offer(
            nft_contract_id.clone().into(),
            account_id.clone(),
            token_id.clone()
        ).expect("Paras: Offer does not exist");

        self.internal_delete_market_data(&nft_contract_id, &token_id);

        ext_contract::nft_transfer_payout(
            offer_data.buyer_id.clone(),
            token_id,
            Some(approval_id),
            Some(U128::from(offer_data.price)),
            Some(10u32), // max length payout
            &nft_contract_id,
            1,
            GAS_FOR_NFT_TRANSFER,
        )
        .then(ext_self::resolve_offer(
            seller_id,
            offer_data,
            &env::current_account_id(),
            NO_DEPOSIT,
            GAS_FOR_ROYALTIES,
        ))
    }

    fn internal_accept_offer_series(
        &mut self,
        nft_contract_id: AccountId,
        account_id: AccountId,
        token_id: TokenId,
        seller_id: AccountId,
        approval_id: u64,
    ) -> Promise {
        // Token delimiter : is specific for Paras NFT

        let mut token_id_iter = token_id.split(":");
        let token_series_id: String = token_id_iter.next().unwrap().parse().unwrap();

        let offer_data = self.internal_delete_offer(
            nft_contract_id.clone().into(),
            account_id.clone(),
            token_series_id.clone()
        ).expect("Paras: Offer does not exist");

        self.internal_delete_market_data(&nft_contract_id, &token_id);

        ext_contract::nft_transfer_payout(
            offer_data.buyer_id.clone(),
            token_id,
            Some(approval_id),
            Some(U128::from(offer_data.price)),
            Some(10u32), // max length payout
            &nft_contract_id,
            1,
            GAS_FOR_NFT_TRANSFER,
        )
            .then(ext_self::resolve_offer(
                seller_id,
                offer_data,
                &env::current_account_id(),
                NO_DEPOSIT,
                GAS_FOR_ROYALTIES,
            ))
    }

    #[private]
    pub fn resolve_offer(
        &mut self,
        seller_id: AccountId,
        offer_data: OfferData
    ) -> U128 {
        let payout_option = promise_result_as_success().and_then(|value| {
            // None means a bad payout from bad NFT contract
            near_sdk::serde_json::from_slice::<Payout>(&value)
                .ok()
                .and_then(|payout| {
                    let mut remainder = offer_data.price;
                    for &value in payout.values() {
                        remainder = remainder.checked_sub(value.0)?;
                    }
                    if remainder == 0 || remainder == 1 {
                        Some(payout)
                    } else {
                        None
                    }
                })
        });
        let payout = if let Some(payout_option) = payout_option {
            payout_option
        } else {
            if offer_data.ft_token_id == "near" {
                Promise::new(offer_data.buyer_id.clone()).transfer(u128::from(offer_data.price));
            }
            // leave function and return all FTs in ft_resolve_transfer
            env::log(
                json!({
                    "type": "resolve_purchase_fail",
                    "params": {
                        "owner_id": seller_id,
                        "nft_contract_id": offer_data.nft_contract_id,
                        "token_id": offer_data.token_id,
                        "ft_token_id": offer_data.ft_token_id,
                        "price": offer_data.price.to_string(),
                        "buyer_id": offer_data.buyer_id,
                    }
                })
                    .to_string()
                    .as_bytes(),
            );
            return offer_data.price.into();
        };

        // Payout (transfer to royalties and seller)
        if offer_data.ft_token_id == "near" {
            // 5% fee for treasury
            let treasury_fee = offer_data.price as u128 * TREASURY_FEE / 10_000u128;

            for (receiver_id, amount) in payout {
                if receiver_id == seller_id {
                    Promise::new(receiver_id).transfer(amount.0 - treasury_fee);
                    Promise::new(self.treasury_id.clone()).transfer(treasury_fee);
                } else {
                    Promise::new(receiver_id).transfer(amount.0);
                }
            }
            env::log(
                json!({
                    "type": "resolve_purchase",
                    "params": {
                        "owner_id": seller_id,
                        "nft_contract_id": offer_data.nft_contract_id,
                        "token_id": offer_data.token_id,
                        "ft_token_id": offer_data.ft_token_id,
                        "price": offer_data.price.to_string(),
                        "buyer_id": offer_data.buyer_id,
                    }
                })
                    .to_string()
                    .as_bytes(),
            );

            return offer_data.price.into();
        } else {
            U128(0)
        }
    }

    // Auction bids
    #[payable]
    pub fn add_bid(
        &mut self,
        nft_contract_id: ValidAccountId,
        ft_token_id: ValidAccountId,
        token_id: TokenId,
        amount: U128
    ) {
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
        let mut market_data = self.market.get(&contract_and_token_id).expect("Paras: Token id does not exist");

        let bidder_id = env::predecessor_account_id();

        let current_time = env::block_timestamp();
        if market_data.started_at.is_some() {
            assert!(current_time >= market_data.started_at.unwrap(), "Paras: Sale has not started yet");
        }

        if market_data.ended_at.is_some() {
            assert!(current_time <= market_data.ended_at.unwrap(), "Paras: Sale has ended");
        }

        assert!(
            env::attached_deposit() >= amount.into(), 
            "Paras: attached deposit is less than amount"
        );

        assert_eq!(
            ft_token_id.to_string(),
            "near",
            "Paras: Only support NEAR"
        );

        assert!(
            market_data.end_price.is_none(),
            "Paras: Dutch auction does not accept add_bid"
        );

        let new_bid = Bid {
            bidder_id: bidder_id.clone(),
            price: amount.into()
        };

        let mut bids = market_data.bids.unwrap_or(Vec::new());

        if !bids.is_empty() {
            let current_bid = &bids[bids.len()-1];

            assert!(
                amount.0 > current_bid.price.0,
                "Paras: Can't pay less than or equal to current bid price: {:?}",
                current_bid.price
            );

            assert!(
                amount.0 > market_data.price,
                "Paras: Can't pay less than or equal to starting price: {:?}",
                U128(market_data.price)
            );

            // refund
            Promise::new(current_bid.bidder_id.clone()).transfer(current_bid.price.0);

            // always keep 1 bid for now
            bids.remove(bids.len()-1);
        } else {
            assert!(
                amount.0 > market_data.price,
                "Paras: Can't pay less than or equal to starting price: {}",
                market_data.price
            );
        }

        bids.push(new_bid);
        market_data.bids = Some(bids);
        self.market.insert(&contract_and_token_id, &market_data);

        env::log(
            json!({
                "type": "add_bid",
                "params": {
                    "bidder_id": bidder_id,
                    "nft_contract_id": nft_contract_id,
                    "token_id": token_id,
                    "ft_token_id": ft_token_id,
                    "amount": amount,
                }
            })
            .to_string()
            .as_bytes(),
        );
    }

    #[payable]
    pub fn accept_bid(
        &mut self,
        nft_contract_id: ValidAccountId,
        token_id: TokenId,
    ) {
        assert_one_yocto();
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
        let mut market_data = self.market.get(&contract_and_token_id).expect("Paras: Token id does not exist");

        assert_eq!(
            market_data.owner_id,
            env::predecessor_account_id(),
            "Paras: Only seller can call accept_bid"
        );

        assert!(
            market_data.end_price.is_none(),
            "Paras: Dutch auction does not accept accept_bid"
        );

        let mut bids = market_data.bids.unwrap();
        let selected_bid = bids.remove(bids.len()-1);
        market_data.bids = Some(bids);
        self.market.insert(&contract_and_token_id, &market_data);

        self.internal_process_purchase(
            market_data.nft_contract_id, 
            token_id, 
            selected_bid.bidder_id.clone(), 
            selected_bid.price.clone().0,
        );
    }


    // Market Data functions

    #[payable]
    pub fn update_market_data(
        &mut self,
        nft_contract_id: ValidAccountId,
        token_id: TokenId,
        ft_token_id: ValidAccountId,
        price: U128,
    ) {
        assert_one_yocto();
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        let mut market_data = self.market.get(&contract_and_token_id).expect("Paras: Token id does not exist ");

        assert_eq!(
            market_data.owner_id,
            env::predecessor_account_id(),
            "Paras: Seller only"
        );

        assert_eq!(
            ft_token_id.to_string(),
            market_data.ft_token_id,
            "Paras: ft_token_id differs"
        ); // sanity check

        market_data.price = price.into();
        self.market.insert(&contract_and_token_id, &market_data);

        env::log(
            json!({
                "type": "update_market_data",
                "params": {
                    "owner_id": market_data.owner_id,
                    "nft_contract_id": nft_contract_id,
                    "token_id": token_id,
                    "ft_token_id": ft_token_id,
                    "price": price,
                }
            })
            .to_string()
            .as_bytes(),
        );
    }

    fn internal_add_market_data(
        &mut self,
        owner_id: ValidAccountId,
        approval_id: u64,
        nft_contract_id: AccountId,
        token_id: TokenId,
        ft_token_id: String,
        price: U128,
        started_at: Option<U64>,
        ended_at: Option<U64>,
        end_price: Option<U128>,
        is_auction: Option<bool>
    ) {
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);

        let bids: Option<Bids> = match is_auction {
            Some(u) => {
                if u {
                    Some(Vec::new())
                }
                else {
                    None
                }
            },
            None => None
        };


        let current_time: u64 = env::block_timestamp();

        if started_at.is_some() {
            assert!(started_at.unwrap().0 >= current_time);

            if ended_at.is_some() {
                assert!(started_at.unwrap().0 < ended_at.unwrap().0);
            }
        }

        if ended_at.is_some() {
            assert!(ended_at.unwrap().0 >= current_time);
        }
        
        if end_price.is_some() {
            assert!(end_price.unwrap().0 < price.0, "Paras: End price is more than starting price");
        }

        self.market.insert(
            &contract_and_token_id,
            &MarketData {
                owner_id: owner_id.clone().into(),
                approval_id,
                nft_contract_id: nft_contract_id.clone().into(),
                token_id: token_id.clone(),
                ft_token_id: ft_token_id.clone(),
                price: price.into(),
                bids: bids,
                started_at: match started_at {
                    Some(x) => Some(x.0),
                    None => None
                },
                ended_at: match ended_at {
                    Some(x) => Some(x.0),
                    None => None
                },
                end_price: match end_price {
                    Some(x) => Some(x.0),
                    None => None
                },
                accept_nft_contract_id: None,
                accept_token_id: None,
                is_auction: is_auction
            },
        );

        let mut token_ids = self.by_owner_id.get(owner_id.as_ref()).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::ByOwnerIdInner {
                    account_id_hash: hash_account_id(owner_id.as_ref()),
                }
                .try_to_vec()
                .unwrap()
            )
        });

        token_ids.insert(&contract_and_token_id);

        self.by_owner_id.insert(&owner_id.to_string(), &token_ids);

        env::log(
            json!({
                "type": "add_market_data",
                "params": {
                    "owner_id": owner_id,
                    "approval_id": approval_id,
                    "nft_contract_id": nft_contract_id,
                    "token_id": token_id,
                    "ft_token_id": ft_token_id,
                    "price": price,
                    "started_at": started_at,
                    "ended_at": ended_at,
                    "end_price": end_price,
                    "is_auction": is_auction
                }
            })
            .to_string()
            .as_bytes(),
        );
    }
    
    fn internal_delete_market_data(
        &mut self,
        nft_contract_id: &AccountId,
        token_id: &TokenId
    ) -> Option<MarketData> {
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
        let market_data = self.market.remove(&contract_and_token_id);

        if let Some(market) = market_data {
            let mut by_owner_id = self.by_owner_id.get(&market.owner_id).expect("No sale by owner_id");
            by_owner_id.remove(&contract_and_token_id);
            if by_owner_id.is_empty() {
                self.by_owner_id.remove(&market.owner_id);
            } else {
                self.by_owner_id.insert(&market.owner_id, &by_owner_id);
            }

            // refund the bids
            if let Some(ref bids) = market.bids {
                for bid in bids {
                    Promise::new(bid.bidder_id.clone()).transfer(bid.price.0);
                }
            }
            Some(market)
        } else {
            None
        }
    }

    #[payable]
    pub fn delete_market_data(
        &mut self,
        nft_contract_id: ValidAccountId,
        token_id: TokenId,
    ) {
        assert_one_yocto();
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        let market_data = self.market.get(&contract_and_token_id).expect("Paras: Token id does not exist ");

        assert!(
            [market_data.owner_id.clone(), self.owner_id.clone()].contains(&env::predecessor_account_id()),
            "Paras: Seller or owner only"
        );

        self.internal_delete_market_data(nft_contract_id.as_ref(), &token_id);
        // what about the approval_id?
        env::log(
            json!({
                "type": "delete_market_data",
                "params": {
                    "owner_id": market_data.owner_id,
                    "nft_contract_id": nft_contract_id,
                    "token_id": token_id,
                }
            })
            .to_string()
            .as_bytes(),
        );
    }

    // Storage

    #[payable]
    pub fn storage_deposit(&mut self, account_id: Option<ValidAccountId>) {
        let storage_account_id = account_id
            .map(|a| a.into())
            .unwrap_or_else(env::predecessor_account_id);
        let deposit = env::attached_deposit();
        assert!(
            deposit >= STORAGE_ADD_MARKET_DATA,
            "Requires minimum deposit of {}",
            STORAGE_ADD_MARKET_DATA
        );

        let mut balance: u128 = self.storage_deposits.get(&storage_account_id).unwrap_or(0);
        balance += deposit;
        self.storage_deposits.insert(&storage_account_id, &balance);
    }

    #[payable]
    pub fn storage_withdraw(&mut self) {
        assert_one_yocto();
        let owner_id = env::predecessor_account_id();
        let mut amount = self.storage_deposits.remove(&owner_id).unwrap_or(0);
        let market_data_owner = self.by_owner_id.get(&owner_id);
        let len = market_data_owner.map(|s| s.len()).unwrap_or_default();
        let diff = u128::from(len) * STORAGE_ADD_MARKET_DATA;
        amount -= diff;
        if amount > 0 {
            Promise::new(owner_id.clone()).transfer(amount);
        }
        if diff > 0 {
            self.storage_deposits.insert(&owner_id, &diff);
        }
    }

    pub fn storage_minimum_balance(&self) -> U128 {
        U128(STORAGE_ADD_MARKET_DATA)
    }

    pub fn storage_balance_of(&self, account_id: ValidAccountId) -> U128 {
        U128(self.storage_deposits.get(account_id.as_ref()).unwrap_or(0))
    }
    
    // View

    pub fn get_market_data(
        self,
        nft_contract_id: ValidAccountId,
        token_id: TokenId,
    ) -> MarketDataJson {
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        let market_data = self.market.get(&contract_and_token_id).expect("Paras: Token id does not exist ");

        let mut price = market_data.price;

        if market_data.is_auction.is_some() && market_data.end_price.is_some() {
            let current_time = env::block_timestamp();
            let end_price = market_data.end_price.unwrap();
            let started_at = market_data.started_at.unwrap();
            let ended_at = market_data.ended_at.unwrap();

            if current_time < started_at {
                // Use current market_data.price
            } else if current_time > ended_at {
                price = end_price;
            } else {
                let time_since_start = current_time - started_at;
                let duration = ended_at - started_at;
                price = price - ((price - end_price) / duration as u128) * time_since_start as u128;
            }
        }

        MarketDataJson{
            owner_id: market_data.owner_id,
            approval_id: market_data.approval_id.into(),
            nft_contract_id: market_data.nft_contract_id,
            token_id: market_data.token_id,
            ft_token_id: market_data.ft_token_id, // "near" for NEAR token
            price: U128(price),
            bids: market_data.bids,
            started_at: match market_data.started_at {
                Some(x) => Some(U64(x)),
                None => None
            },
            ended_at: match market_data.ended_at {
                Some(x) => Some(U64(x)),
                None => None
            },
            end_price: match market_data.end_price {
                Some(x) => Some(U128(x)),
                None => None
            },
            is_auction: market_data.is_auction,
        }
    }

    pub fn approved_ft_token_ids(&self) -> Vec<AccountId> {
        self.approved_ft_token_ids.to_vec()
    }

    pub fn approved_nft_contract_ids(&self) -> Vec<AccountId> {
        self.approved_nft_contract_ids.to_vec()
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    pub fn get_treasury(&self) -> AccountId {
        self.treasury_id.clone()
    }

    pub fn get_supply_by_owner_id(&self, account_id: AccountId) -> U64 {
        let by_owner_id = self.by_owner_id.get(&account_id);
        if let Some(by_owner_id) = by_owner_id {
            U64(by_owner_id.len())
        } else {
            U64(0)
        }
    }
}

pub fn hash_account_id(account_id: &AccountId) -> CryptoHash {
        let mut hash = CryptoHash::default();
        hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
        hash
}

#[ext_contract(ext_self)]
trait ExtSelf {
    fn resolve_purchase(
        &mut self,
        buyer_id: AccountId,
        market_data: MarketData,
        price: U128,
    ) -> Promise;

    fn resolve_offer(
        &mut self,
        seller_id: AccountId,
        offer_data: OfferData
    ) -> Promise;
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env};
    use std::convert::TryFrom;

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn setup_contract() -> (VMContextBuilder, Contract) {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        let contract = Contract::new(
            accounts(0),
            accounts(1),
            None,
            Some(vec![accounts(2)]),
            Some(vec![accounts(2)])
        );
        (context, contract)
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let contract = Contract::new(
            accounts(0),
            accounts(1),
            None,
            Some(vec![accounts(2)]),
            Some(vec![accounts(2)]),
        );
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.get_owner(), accounts(0).to_string());
        assert_eq!(contract.get_treasury(), accounts(1).to_string());
        assert_eq!(contract.approved_ft_token_ids(), vec!["near"]);
        assert_eq!(contract.approved_nft_contract_ids(), vec![accounts(2).to_string()]);
    }

    #[test]
    fn test_set_treasury() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build()
        );

        contract.set_treasury(accounts(5));
        let new_treasury: AccountId = contract.get_treasury();
        assert_eq!(new_treasury, accounts(5).to_string());
    }

    #[test]
    #[should_panic(expected = "Paras: Owner only")]
    fn test_invalid_set_treasury() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(1)
            .build()
        );

        contract.set_treasury(accounts(5));
    }

    #[test]
    fn test_transfer_ownership() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build()
        );

        contract.transfer_ownership(accounts(5));
        let new_owner: AccountId = contract.get_owner();
        assert_eq!(new_owner, accounts(5).to_string());
    }

    #[test]
    #[should_panic(expected = "Paras: Owner only")]
    fn test_invalid_transfer_ownership() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(5))
            .attached_deposit(1)
            .build()
        );

        contract.transfer_ownership(accounts(5));
    }

    #[test]
    fn test_add_approved_ft_token_ids() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build()
        );

        contract.add_approved_ft_token_ids(vec![accounts(5)]);
        let approved_fts = contract.approved_ft_token_ids();
        assert_eq!(
            approved_fts,
            vec!["near".to_string(), accounts(5).to_string()]
        );
    }

    #[test]
    fn test_add_approved_nft_contract_ids() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build()
        );

        contract.add_approved_nft_contract_ids(vec![accounts(5)]);
        let approved_nfts = contract.approved_nft_contract_ids();
        assert_eq!(
            approved_nfts,
            vec![accounts(2).to_string(), accounts(5).to_string()]
        );
    }

    #[test]
    fn test_internal_add_market_data() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .build()
        );

        contract.internal_add_market_data(
            accounts(3),
            1,
            accounts(2).to_string(),
            "1:1".to_string(),
            "near".to_string(),
            U128::from(1 * 10u128.pow(24)),
            None,
            None,
            None,
            None
        );

        let market = contract.get_market_data(accounts(2), "1:1".to_string());
        assert_eq!(market.owner_id, accounts(3).to_string());
        assert_eq!(market.approval_id, U64::from(1));
        assert_eq!(market.ft_token_id, "near".to_string());
        assert_eq!(market.nft_contract_id, accounts(2).to_string());
        assert_eq!(market.owner_id, accounts(3).to_string());
        assert_eq!(market.token_id, "1:1".to_string());
        assert_eq!(market.price, U128::from(1 * 10u128.pow(24)));
    }

    #[test]
    #[should_panic(expected = "Paras: Seller only")]
    fn test_invalid_update_market_data() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .build()
        );

        contract.internal_add_market_data(
            accounts(3),
            1,
            accounts(2).to_string(),
            "1:1".to_string(),
            "near".to_string(),
            U128::from(1 * 10u128.pow(24)),
            None,
            None,
            None,
            None
        );

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build()
        );

        contract.update_market_data(
            accounts(2),
            "1:1".to_string(),
            ValidAccountId::try_from("near").unwrap(),
            U128::from(2 * 10u128.pow(24)),
        );
    }

    #[test]
    fn test_update_market_data() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .build()
        );

        contract.internal_add_market_data(
            accounts(3),
            1,
            accounts(2).to_string(),
            "1:1".to_string(),
            "near".to_string(),
            U128::from(1 * 10u128.pow(24)),
            None,
            None,
            None,
            None,
        );

        testing_env!(context
            .predecessor_account_id(accounts(3))
            .attached_deposit(1)
            .build()
        );

        contract.update_market_data(
            accounts(2),
            "1:1".to_string(),
            ValidAccountId::try_from("near").unwrap(),
            U128::from(2 * 10u128.pow(24)),
        );

        let market = contract.get_market_data(accounts(2), "1:1".to_string());
        assert_eq!(market.price, U128::from(2 * 10u128.pow(24)));

    }

    #[test]
    #[should_panic(expected = "Paras: Token id does not exist")]
    fn test_delete_market_data() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .build()
        );

        contract.internal_add_market_data(
            accounts(3),
            1,
            accounts(2).to_string(),
            "1:1".to_string(),
            "near".to_string(),
            U128::from(1 * 10u128.pow(24)),
            None,
            None,
            None,
            None
        );

        testing_env!(context
            .predecessor_account_id(accounts(3))
            .attached_deposit(1)
            .build()
        );

        contract.delete_market_data(
            accounts(2),
            "1:1".to_string(),
        );

        contract.get_market_data(accounts(2), "1:1".to_string());
    }

    #[test]
    fn test_storage_deposit() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(STORAGE_ADD_MARKET_DATA)
            .build()
        );

        contract.storage_deposit(None);

        let storage_balance = contract.storage_balance_of(accounts(0)).0;
        assert_eq!(STORAGE_ADD_MARKET_DATA, storage_balance);

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build()
        );

        contract.storage_withdraw();

        let storage_balance = contract.storage_balance_of(accounts(0)).0;
        assert_eq!(0, storage_balance);
    }

    #[test]
    fn test_add_offer() {
        let (mut context, mut contract) = setup_contract();

        let one_near = 10u128.pow(24);

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(one_near)
            .build()
        );

        contract.internal_add_offer(
            accounts(3).to_string(),
            Some("1:1".to_string()),
            None,
            "near".to_string(),
            U128(one_near),
            accounts(0).to_string()
        );

        let offer_data = contract.get_offer(
            accounts(3),
            accounts(0),
            Some("1:1".to_string()),
            None
        );

        assert_eq!(offer_data.buyer_id, accounts(0).to_string());
        assert_eq!(offer_data.price, U128(one_near));
    }

    #[test]
    #[should_panic(expected = "Paras: Offer does not exist")]
    fn test_delete_offer() {
        let (mut context, mut contract) = setup_contract();

        let one_near = 10u128.pow(24);

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(one_near)
            .build()
        );

        contract.internal_add_offer(
            accounts(3).to_string(),
            Some("1:1".to_string()),
            None,
            "near".to_string(),
            U128(one_near),
            accounts(0).to_string()
        );

        contract.delete_offer(
            accounts(3),
            Some("1:1".to_string()),
            None
        );

        contract.get_offer(
            accounts(3),
            accounts(1),
            Some("1:1".to_string()),
            None
        );
    }

    #[test]
    fn test_internal_add_market_data_auction() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .build()
        );

        contract.internal_add_market_data(
            accounts(3),
            1,
            accounts(2).to_string(),
            "1:1".to_string(),
            "near".to_string(),
            U128::from(1 * 10u128.pow(24)),
            None,
            None,
            None,
            Some(true)
        );

        let market = contract.get_market_data(accounts(2), "1:1".to_string());
        assert_eq!(market.is_auction, Some(true));
    }

    #[test]
    #[should_panic(expected = "Paras: the NFT is on auction")]
    fn test_bid_invalid_purchase(){
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .build()
        );

        contract.internal_add_market_data(
            accounts(3),
            1,
            accounts(2).to_string(),
            "1:1".to_string(),
            "near".to_string(),
            U128::from(1 * 10u128.pow(24)),
            None,
            None,
            None,
            Some(true),
        );

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(10u128.pow(24))
            .build()
        );

        contract.buy(
            accounts(2),
            "1:1".to_string()
        );
    }

    #[test]
    fn test_add_bid_and_accept() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .build()
        );

        contract.internal_add_market_data(
            accounts(0),
            1,
            accounts(2).to_string(),
            "1:1".to_string(),
            "near".to_string(),
            U128::from(1 * 10u128.pow(24)),
            None,
            None,
            None,
            Some(true),
        );

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(10u128.pow(24) + 1)
            .build()
        );

        contract.add_bid(
            accounts(2),
            ValidAccountId::try_from("near").unwrap(),
            "1:1".to_string(),
            U128::from(10u128.pow(24) + 1)
        );


        testing_env!(context
            .predecessor_account_id(accounts(4))
            .attached_deposit(10u128.pow(24) + 2)
            .build()
        );

        contract.add_bid(
            accounts(2),
            ValidAccountId::try_from("near").unwrap(),
            "1:1".to_string(),
            U128::from(10u128.pow(24) + 2)
        );

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build()
        );

        contract.accept_bid(accounts(2), "1:1".to_string());
    }
}
