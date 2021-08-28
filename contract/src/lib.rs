use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128, U64, ValidAccountId};
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey,
    PanicOnDefault, serde_json::json, assert_one_yocto, ext_contract, Gas, Balance, Promise
};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::promise_result_as_success;


use crate::external::*;


mod external;
mod nft_callbacks;

const GAS_FOR_NFT_TRANSFER: Gas = 15_000_000_000_000;
const GAS_FOR_ROYALTIES: Gas = 115_000_000_000_000;
const NO_DEPOSIT: Balance = 0;

near_sdk::setup_alloc!();

pub type ContractAndTokenId = String;
pub type TokenId = String;

const DELIMETER: &str = "||";

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MarketData {
    pub owner_id: AccountId,
    pub approval_id: u64,
    pub nft_contract_id: String,
    pub token_id: TokenId,
    pub ft_token_id: AccountId, // "near" for NEAR token
    pub price: u128,
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
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub treasury_id: AccountId,
    pub market: UnorderedMap<ContractAndTokenId, MarketData>,
    pub approved_ft_token_ids: UnorderedSet<AccountId>,
    pub approved_nft_contract_ids: UnorderedSet<AccountId>,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Market,
    FTTokenIds,
    NFTContractIds
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new(
        owner_id: ValidAccountId, 
        treasury_id: ValidAccountId,
        approved_ft_token_ids: Option<Vec<ValidAccountId>>,
        approved_nft_contract_ids: Option<Vec<ValidAccountId>>,
    ) -> Self {
        let mut this = Self {
            owner_id: owner_id.into(),
            treasury_id: treasury_id.into(),
            market: UnorderedMap::new(StorageKey::Market),
            approved_ft_token_ids: UnorderedSet::new(StorageKey::FTTokenIds),
            approved_nft_contract_ids: UnorderedSet::new(StorageKey::NFTContractIds),
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
        let market_data = self.market.get(&contract_and_token_id).expect("Paras: Token id does not exist ");
        let buyer_id = env::predecessor_account_id();

        assert_ne!(
            buyer_id,
            market_data.owner_id,
            "Paras: Cannot buy your own sale"
        );

        assert!(
            env::attached_deposit() >= market_data.price,
            "Paras: Attached deposit is not sufficient"
        );

        // only NEAR supported for now
        assert_eq!(
            market_data.ft_token_id,
            "near",
            "Paras: NEAR support only"
        );

        self.internal_process_purchase(
            nft_contract_id.into(),
            token_id,
            buyer_id,
        );
    }

    fn internal_process_purchase(
        &mut self,
        nft_contract_id: AccountId,
        token_id: TokenId,
        buyer_id: AccountId,
    ) -> Promise {

        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
        let market_data = self.market.remove(&contract_and_token_id).expect("No sale");


        ext_contract::nft_transfer(
            buyer_id.clone(),
            token_id,
            market_data.approval_id.into(),
            &nft_contract_id,
            1,
            GAS_FOR_NFT_TRANSFER,
        )
        .then(ext_self::resolve_purchase(
            buyer_id,
            market_data,
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
    ) -> U128 {
        let result = promise_result_as_success();
        if result.is_none() {
            if market_data.ft_token_id == "near" {
                Promise::new(buyer_id).transfer(u128::from(market_data.price));
            }
            return market_data.price.into();
        } else {
            if market_data.ft_token_id == "near" {
                Promise::new(market_data.owner_id).transfer(market_data.price);
                // refund all FTs (won't be any)
                return market_data.price.into();
            } else {
                U128(0)
            }
        }
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
    ) {
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        self.market.insert(
            &contract_and_token_id,
            &MarketData {
                owner_id: owner_id.clone().into(),
                approval_id,
                nft_contract_id: nft_contract_id.clone().into(),
                token_id: token_id.clone(),
                ft_token_id: ft_token_id.clone(),
                price: price.into(),
            },
        );

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
                }
            })
            .to_string()
            .as_bytes(),
        );
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
            [market_data.owner_id, self.owner_id.clone()].contains(&env::predecessor_account_id()),
            "Paras: Seller or owner only"
        );

        self.market.remove(&contract_and_token_id);
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
    
    // View

    pub fn get_market_data(
        self,
        nft_contract_id: ValidAccountId,
        token_id: TokenId,
    ) -> MarketDataJson {
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        let market_data = self.market.get(&contract_and_token_id).expect("Paras: Token id does not exist ");
        MarketDataJson{
            owner_id: market_data.owner_id,
            approval_id: market_data.approval_id.into(),
            nft_contract_id: market_data.nft_contract_id,
            token_id: market_data.token_id,
            ft_token_id: market_data.ft_token_id, // "near" for NEAR token
            price: market_data.price.into(),
        }
    }

    pub fn supported_ft_token_ids(&self) -> Vec<AccountId> {
        self.approved_ft_token_ids.to_vec()
    }

    pub fn supported_nft_contract_ids(&self) -> Vec<AccountId> {
        self.approved_nft_contract_ids.to_vec()
    }
}

#[ext_contract(ext_self)]
trait ExtSelf {
    fn resolve_purchase(
        &mut self,
        buyer_id: AccountId,
        market_data: MarketData,
    ) -> Promise;
}