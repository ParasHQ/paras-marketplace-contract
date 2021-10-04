# Payment test

## NFT deploy
```
near call --accountId gnaor.testnet dev-1630163823356-82615235483639 new_default_meta '{"owner_id":"gnaor.testnet", "treasury_id": "johnnear.testnet"}'
```

## NFT create series
```
near call --accountId gnaor.testnet dev-1630163823356-82615235483639 nft_create_series '{"token_series_id":"2","creator_id":"orang.testnet","token_metadata":{"title":"Naruto Shippuden ch.2: Menolong sasuke","media":"bafybeidzcan4nzcz7sczs4yzyxly4galgygnbjewipj6haco4kffoqpkiy", "reference":"bafybeicg4ss7qh5odijfn2eogizuxkrdh3zlv4eftcmgnljwu7dm64uwji"},"price":"1000000000000000000000000", "royalty":{"orang.testnet": 9000}}' --depositYocto 11790000000000000000000
```

## NFT buy (mint)
```
near call --networkId testnet --accountId gnaor.testnet dev-1630163823356-82615235483639 nft_buy '{"token_series_id":"
2","receiver_id":"gnaor.testnet"}' --depositYocto 1018320000000000000000000
```

## Marketplace deploy
```
near call --accountId dev-1630164914800-20665050242974 dev-1630164914800-20665050242974 new '{"treasury_id":"johnnear.testnet","owner_id":"dev-1630164914800-20665050242974","approved_nft_contract_ids":["dev-1630163823356-82615235483639"]}'
```

### Explanation
```
    Price : 3 N
    Treasury Fee: 0.15 N -> to johnnear.testnet
    90% royalty: 2.7 N  -> orang.testnet (creator/artist)
    5% - Treasury Fee: 0.15 N  -> gnaor.testnet (seller)

    buyer: cymac.testnet
```

## Storage deposit
```
near call dev-1630164914800-20665050242974 storage_deposit '{"accountId":"gnaor.testnet"}' --accountId gnaor.testnet --depositYocto 8590000000000000000000
```
## NFT sell 
```
near call --accountId gnaor.testnet dev-1630163823356-82615235483639 nft_approve '{"token_id":"2:1","account_id":"dev-1630164914800-20665050242974","msg":"{\"market_type\":"sale",\"price\":\"3000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --depositYocto 1320000000000000000000
```

## Get market data
```
near view dev-1630164914800-20665050242974 get_market_data '{"nft_contract_id":"dev-1630163823356-82615235483639","token_id":"2:1"}'
```

#### Result
```
View call: dev-1630164914800-20665050242974.get_market_data({"nft_contract_id":"dev-1630163823356-82615235483639","token_id":"2:1"})
{
  owner_id: 'gnaor.testnet',
  approval_id: '1',
  nft_contract_id: 'dev-1630163823356-82615235483639',
  token_id: '2:1',
  ft_token_id: 'near',
  price: '3000000000000000000000000'
}
```

## NFT buy
```
near call --accountId cymac.testnet dev-1630164914800-20665050242974 buy '{"nft_contract_id":"dev-1630163823356-82615235483639","token_id":"2:1"}' --deposit 3 --gas 160000000000000
```

#### Result
```
$ near --accountId cymac.testnet call dev-1630164914800-20665050242974 buy '{"nft_contract_id":"dev-1630163823356-82615235483639","token_id":"2:1"}' --deposit 3 --gas 160000000000000
Scheduling a call: dev-1630164914800-20665050242974.buy({"nft_contract_id":"dev-1630163823356-82615235483639","token_id":"2:1"}) with attached 3 NEAR
Receipt: Fy7X3SpYccwKqcmStGnafumrqWNggrhXdiwywpMmkzag
        Log [dev-1630164914800-20665050242974]: Transfer 2:1 from dev-1630164914800-20665050242974 to cymac.testnet
        Log [dev-1630164914800-20665050242974]: {"type":"nft_transfer","params":{"token_id":"2:1","sender_id":"gnaor.testnet","receiver_id":"cymac.testnet"}}
Transaction Id Gaj1nXYNnRoBh2qhdcArCDdj7UAAtZtwiAMZAp5GAcyN
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/Gaj1nXYNnRoBh2qhdcArCDdj7UAAtZtwiAMZAp5GAcyN
''
```
