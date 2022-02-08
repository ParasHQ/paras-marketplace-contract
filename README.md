# NFT Marketplace Implementation

## Instructions

`yarn && yarn test:deploy`

#### Pre-reqs

Rust, cargo, near-cli, etc...
Everything should work if you have NEAR development env for Rust contracts set up.

[Tests](test/api.test.js)
[Contract](contract/src/lib.rs)

## Example Call

### Deploy
```
near deploy --accountId marketplace.test.near
```

### Nft init
```
near call --accountId marketplace.test.near marketplace.test.near new '{"owner_id":"marketplace.test.near","treasury_id":"treasury.test.near","approved_nft_contract_ids":["comic.test.near"],"paras_nft_contracts":["comic.test.near"],"current_fee":500}'
```

### Set transaction fee (owner only)
```
near call --accountId marketplace.test.near marketplace.test.near set_transaction_fee '{"next_fee":500,"start_time":1644311100}'
```

### Get transaction fee
```
near call --accountId marketplace.test.near marketplace.test.near get_transaction_fee
```

### Nft sell (to NFT contract)
```
near call --accountId alice.test.near comic.test.near nft_approve '{"token_id":"1:10","account_id":"marketplace.test.near","msg":"{\"price\":\"3000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --depositYocto 2610000000000000000000
```

### Delete market data
```
near call --accountId alice.test.near marketplace.test.near delete_market_data '{"nft_contract_id":"comic.test.near", "token_id":"1:2"}' --depositYocto 1
```

### Update market data
```
near call --accountId alice.test.near marketplace.test.near update_market_data '{"nft_contract_id":"comic.test.near", "token_id":"1:2", "ft_token_id":"near","price":"5000000000000000000000000"}' --depositYocto 1
```

### Buy
```
near call --accountId bob.test.near marketplace.test.near buy '{"nft_contract_id":"comic.test.near","token_id":"1:10"}' --depositYocto 3000000000000000000000000 --gas 300000000000000
```

## View

### Get market data
```
near view marketplace.test.near get_market_data '{"nft_contract_id":"comic.test.near","token_id":"1:10"}'
```