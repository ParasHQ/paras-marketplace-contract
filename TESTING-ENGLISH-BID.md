# Payment test

## NFT deploy
```
near call --accountId gnaor.testnet dev-1633511167301-22856066114973 new_default_meta '{"owner_id":"gnaor.testnet", "treasury_id": "johnnear.testnet"}'
```

## NFT create series
```
near call --accountId gnaor.testnet dev-1633511167301-22856066114973 nft_create_series '{"creator_id":"orang.testnet","token_metadata":{"title":"Naruto Shippuden ch.2: Menolong sasuke","media":"bafybeidzcan4nzcz7sczs4yzyxly4galgygnbjewipj6haco4kffoqpkiy", "reference":"bafybeicg4ss7qh5odijfn2eogizuxkrdh3zlv4eftcmgnljwu7dm64uwji"},"price":"1000000000000000000000000", "royalty":{"orang.testnet": 9000}}' --depositYocto 11790000000000000000000
```

## NFT buy (mint)
```
near call --networkId testnet --accountId gnaor.testnet dev-1633511167301-22856066114973 nft_buy '{"token_series_id":"
2","receiver_id":"gnaor.testnet"}' --depositYocto 1018320000000000000000000
```

## Marketplace deploy
```
near call --accountId dev-1633509841439-11593553362203 dev-1633509841439-11593553362203 new '{"treasury_id":"johnnear.testnet","owner_id":"dev-1633509841439-11593553362203","approved_nft_contract_ids":["dev-1633511167301-22856066114973"], "paras_nft_contract": "dev-1633511167301-22856066114973"}'
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
near call dev-1633509841439-11593553362203 storage_deposit '{"accountId":"gnaor.testnet"}' --accountId gnaor.testnet --depositYocto 8590000000000000000000
```

## NFT add market data auction 
```
near call --accountId gnaor.testnet dev-1633511167301-22856066114973 nft_approve '{"token_id":"1:1","account_id":"dev-1633509841439-11593553362203","msg":"{\"market_type\":\"sale\",\"price\":\"3000000000000000000000000\",\"ft_token_id\":\"near\",\"is_auction\":true}"}' --depositYocto 1320000000000000000000
```

## Get market data
```
near view dev-1633509841439-11593553362203 get_market_data '{"nft_contract_id":"dev-1633511167301-22856066114973", "token_id":"1:1"}'
View call: dev-1633509841439-11593553362203.get_market_data({"nft_contract_id":"dev-1633511167301-22856066114973", "token_id":"1:1"})
{
  owner_id: 'gnaor.testnet',
  approval_id: '6',
  nft_contract_id: 'dev-1633511167301-22856066114973',
  token_id: '1:1',
  ft_token_id: 'near',
  price: '3000000000000000000000000',
  bids: null,
  started_at: null,
  ended_at: null,
  end_price: null,
  is_auction: true
}near view dev-1633509841439-11593553362203 get_market_data '{"nft_contract_id":"dev-1633511167301-22856066114973", "token_id":"1:1"}'
```

## NFT add bid
```
near call dev-1633509841439-11593553362203 add_bid '{"nft_contract_id":"dev-1633511167301-22856066114973", "token_id":"1:1", "ft_token_id": "near", "amount":"3000000000000000000000002"}' --depositYocto 3000000000000000000000002 --accountId projectp.testnet
Scheduling a call: dev-1633509841439-11593553362203.add_bid({"nft_contract_id":"dev-1633511167301-22856066114973", "token_id":"1:1", "ft_token_id": "near", "amount":"3000000000000000000000002"}) with attached 3.000000000000000000000002 NEAR
Receipts: 5GUQt72QTkEf6MoV6NZskGdSDmVMQd6jZNtb1tyRgp8X, 9mFmJ6MgbmsaYFpkkGhhSpNtm85gUg3LxeDjQSPdWUT1
	Log [dev-1633509841439-11593553362203]: {"type":"add_bid","params":{"bidder_id":"projectp.testnet","nft_contract_id":"dev-1633511167301-22856066114973","token_id":"1:1","ft_token_id":"near","amount":"3000000000000000000000002"}}
Transaction Id 78AXk46Zyi1Er7yKXSGFqDYu4yWJGpU3GssPVqrL8LtE
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/78AXk46Zyi1Er7yKXSGFqDYu4yWJGpU3GssPVqrL8LtE
''
```

## NFT accept bid
```
$ near call dev-1633509841439-11593553362203 accept_bid '{"nft_contract_id":"dev-1633511167301-22856066114973", "token_id":"1:3"}' --depositYocto 1 --accountId gnaor.testnet --gas 100000000000000                                                             
Scheduling a call: dev-1633509841439-11593553362203.accept_bid({"nft_contract_id":"dev-1633511167301-22856066114973", "token_id":"1:3"}) with attached 0.000000000000000000000001 NEAR
Receipt: CHDHyoiCdMJT7E1Ag3ach9tJr7yudrhzbXed1rXgn1eW
	Log [dev-1633509841439-11593553362203]: Transfer 1:3 from dev-1633509841439-11593553362203 to projectp.testnet
	Log [dev-1633509841439-11593553362203]: {"type":"nft_transfer","params":{"token_id":"1:3","sender_id":"gnaor.testnet","receiver_id":"projectp.testnet"}}
Receipts: 2X7M4mo8bGEdQJsB4bxADVLPj4reFmjx5uB1WGE9zzsu, BYLrPAdsxCdKtsj1YGAKCfZmK2rYpCowpkVzC5y7zB9Y, 59ugE85wbFdpxREnYg9MyHfE4ZSbhaUWNFQrfCaDa6kS, 3gqijGDHWebH5zWrWh4h7AMVfMFLjg9A5QjPhcKwqF9H
	Log [dev-1633509841439-11593553362203]: {"type":"resolve_purchase","params":{"owner_id":"gnaor.testnet","nft_contract_id":"dev-1633511167301-22856066114973","token_id":"1:3","ft_token_id":"near","price":"300000000000000000000003","buyer_id":"projectp.testnet"}}
Transaction Id 3orGT2eTHYVP47HcgWYpwgX8mALNLLfisyW5rga2Cvg2
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/3orGT2eTHYVP47HcgWYpwgX8mALNLLfisyW5rga2Cvg2
''
```