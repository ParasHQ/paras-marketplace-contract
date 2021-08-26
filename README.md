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
env NEAR_ENV=local near --keyPath ~/.near/localnet/validator_key.json deploy --accountId marketplace.test.near
```

### Nft init
```
env NEAR_ENV=local near call --keyPath ~/.near/localnet/validator_key.json --accountId marketplace.test.near marketplace.test.near new '{"owner_id":"marketplace.test.near","approved_nft_contract_ids":["comic.test.near"]}'
```

### Nft sell (to NFT contract)
```
env NEAR_ENV=local near call --keyPath ~/.near/localnet/validator_key.json --accountId alice.test.near comic.test.near nft_approve '{"token_id":"1:10","account_id":"marketplace.test.near","msg":"{\"price\":\"3000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --depositYocto 1320000000000000000000
```

### Buy
```
env NEAR_ENV=local near --keyPath ~/.near/localnet/validator_key.json call --accountId bob.test.near marketplace.test.near buy '{"nft_contract_id":"comic.test.near","token_id":"1:10"}' --depositYocto 3000000000000000000000000 --gas 300000000000000
```

## View

### Get market data
```
env NEAR_ENV=local near --keyPath ~/.near/localnet/validator_key.json view marketplace.test.near get_market_data '{"nft_contract_id":"comic.test.near","token_id":"1:10"}'
```