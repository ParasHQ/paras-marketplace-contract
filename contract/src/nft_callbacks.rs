use crate::*;

/// approval callbacks from NFT Contracts

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MartketArgs {
    pub price: U128,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ft_token_id: Option<AccountId>,
}

trait NonFungibleTokenApprovalsReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: ValidAccountId,
        approval_id: u64,
        msg: String,
    );
}

#[near_bindgen]
impl NonFungibleTokenApprovalsReceiver for Contract {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: ValidAccountId,
        approval_id: u64,
        msg: String,
    ) {
        // enforce cross contract call and owner_id is signer

        let nft_contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();
        assert_ne!(
            nft_contract_id,
            signer_id,
            "Paras: nft_on_approve should only be called via cross-contract call"
        );
        assert_eq!(
            owner_id.as_ref(),
            &signer_id,
            "Paras: owner_id should be signer_id"
        );

        assert!(
            self.approved_nft_contract_ids.contains(&nft_contract_id),
            "Paras: nft_contract_id is not approved"
        );

        let MartketArgs { price, ft_token_id } =
            near_sdk::serde_json::from_str(&msg).expect("Not valid MarketArgs");

        
        let ft_token_id_res: AccountId;
        if let Some(ft_contract_id) = ft_token_id {
            ft_token_id_res = ft_contract_id;
        } else {
            ft_token_id_res = String::from("near");
        }

        if self.approved_ft_token_ids.contains(&ft_token_id_res) != true {
            env::panic("Paras: ft_token_id not supported".as_bytes());
        }

        self.internal_add_market_data(
            owner_id, 
            approval_id, 
            nft_contract_id, 
            token_id, 
            ft_token_id_res, 
            price
        );
    }
}
