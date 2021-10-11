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
near call --accountId dev-1633509841439-11593553362203 dev-1633509841439-11593553362203 new '{"treasury_id":"johnnear.testnet","owner_id":"dev-1633509841439-11593553362203","approved_nft_contract_ids":["dev-1630163823356-82615235483639"]}'
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
near call dev-1633509841439-11593553362203 storage_deposit '{"accountId":"cymac.testnet"}' --accountId cymac.testnet --depositYocto 8590000000000000000000
```
## NFT add offer 
```
near call dev-1633509841439-11593553362203 add_offer '{"nft_contract_id":"dev-1633337760334-37276932331579", "token_id":"1:2", "ft_token_id":"near", "price": "1000000000000000000000000"}' --accountId cymac.testnet --deposit 1
```

## Get offer
```
$ near view dev-1633509841439-11593553362203 get_offer '{"nft_contract_id":"dev-1633337760334-37276932331579","token_id":"1:2","account_id":"cymac.testnet"}'
View call: dev-1633509841439-11593553362203.get_offer({"nft_contract_id":"dev-1633337760334-37276932331579","token_id":"1:2","account_id":"cymac.testnet"})
{
  buyer_id: 'cymac.testnet',
  nft_contract_id: 'dev-1633337760334-37276932331579',
  token_id: '1:2',
  token_series_id: null,
  ft_token_id: 'near',
  price: '1000000000000000000000000'
}```

## NFT accept offer
```
$ near call --accountId gnaor.testnet  dev-1633337760334-37276932331579  nft_approve '{"token_id":"1:12","account_id":"dev-1633509841439-11593553362203","msg":"{\"market_type\":\"accept_offer\",\"account_id\":\"cymac.testnet\"}"}' --depositYocto 440000000000000000000 --gas 300000000000000
Scheduling a call: dev-1633337760334-37276932331579.nft_approve({"token_id":"1:12","account_id":"dev-1633509841439-11593553362203","msg":"{\"market_type\":\"accept_offer\",\"buyer_id\":\"cymac.testnet\"}"}) with attached 0.00044 NEAR
Receipt: 8yAsQodQDbG7U5VrsYt56K34KNMtycDyA6j9MvSoizgt
Log [dev-1633337760334-37276932331579]: Transfer 1:12 from dev-1633509841439-11593553362203 to cymac.testnet
Log [dev-1633337760334-37276932331579]: {"type":"nft_transfer","params":{"token_id":"1:12","sender_id":"gnaor.testnet","receiver_id":"cymac.testnet"}}
Receipts: AXPqmu3HtUMLMooNahar21JUJzyJ1UxyJRAWHtoSVM9F, 8g7QMhJBYMnk44gmdz1Wok8PSpbbNHXS5rEDSxFnPTQs, 4QMpUz34dThuxKKfGvQ1iVbUxH2EpsZ2rQ3BYmxnyJ8m, J5QrETWsJ4CG3xiueqwEeUoS6vGUoNYGgndU8RfMchqb
Log [dev-1633337760334-37276932331579]: {"type":"resolve_purchase","params":{"owner_id":"gnaor.testnet","nft_contract_id":"dev-1633337760334-37276932331579","token_id":"1:12","ft_token_id":"near","price":"100000000000000000000000","buyer_id":"cymac.testnet"}}
Transaction Id mhfKCh6RxGSwcAohEwMZC2VUv3kteqoGfs5APiGL1aP
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/mhfKCh6RxGSwcAohEwMZC2VUv3kteqoGfs5APiGL1aP
```

## NFT accept offer Paras series NFT
```
$ near call --accountId gnaor.testnet dev-1633337760334-37276932331579 nft_approve '{"token_id":"1:15","account_id":"dev-1633354630145-57340825824805","msg":"{\"market_type\":\"accept_offer_paras_series\",\"account_id\":\"cymac.testnet\"}"}' --depositYocto 440000000000000000000 --gas 300000000000000
Scheduling a call: dev-1633337760334-37276932331579.nft_approve({"token_id":"1:15","account_id":"dev-1633354630145-57340825824805","msg":"{\"market_type\":\"accept_offer_paras_series\",\"buyer_id\":\"cymac.testnet\"}"}) with attached 0.00044 NEAR
Retrying request to broadcast_tx_commit as it has timed out [
'DQAAAGduYW9yLnRlc3RuZXQA1ppnEtFtCLv528lk5cawPJOIuphC0tVOpkvnRGSqibhrAQAAAAAAACAAAABkZXYtMTYzMzMzNzc2MDMzNC0zNzI3NjkzMjMzMTU3Obb0KrkP+AwBHel56Ao9MpMYauNvHHnXDoUALG5jno5ZAQAAAAILAAAAbmZ0X2FwcHJvdmWcAAAAeyJ0b2tlbl9pZCI6IjE6MTUiLCJhY2NvdW50X2lkIjoiZGV2LTE2MzMzNTQ2MzAxNDUtNTczNDA4MjU4MjQ4MDUiLCJtc2ciOiJ7XCJtYXJrZXRfdHlwZVwiOlwiYWNjZXB0X29mZmVyX3BhcmFzX3Nlcmllc1wiLFwiYWNjb3VudF9pZFwiOlwiY3ltYWMudGVzdG5ldFwifSJ9AMBuMdkQAQAAAOCzxwQ62hcAAAAAAAAAAP1iAMGljBH81cqnlrJhuKfTdC5kKbgsJOAupMt6wqK2WfCUQ6fXVkz7YOqVpICcrBZ4iRqOdybfBpILa3D5yAU='
]
Receipt: 14aDLXPZ8M1cuaj9C6zTY93nc8qDds2R1zqFhYrVT4WY
Log [dev-1633337760334-37276932331579]: Transfer 1:15 from dev-1633354630145-57340825824805 to cymac.testnet
Log [dev-1633337760334-37276932331579]: {"type":"nft_transfer","params":{"token_id":"1:15","sender_id":"gnaor.testnet","receiver_id":"cymac.testnet"}}
Receipts: 4jHAor7BMfBc327KUkHvfgQGc7AR6pe9fzxernzni6vA, 7vSHhX9uUz9YLupXziwzUQwhCy2Xtp9YmJdqzfA37B8z, EekV83NNHuSveEQLAobh6TLAvwhgiP3ojYSMzi7fkRfp, FEEXCaG7oxdpNY857nzdeixvdujwJN6QTnBAdLayXbL4
Log [dev-1633337760334-37276932331579]: {"type":"resolve_purchase","params":{"owner_id":"gnaor.testnet","nft_contract_id":"dev-1633337760334-37276932331579","token_id":null,"ft_token_id":"near","price":"100000000000000000000000","buyer_id":"cymac.testnet"}}
Transaction Id 3BPCzaBFCLTKMUuFT7JbtVMwaEspNvH6o1gxubB5Z5pG
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/3BPCzaBFCLTKMUuFT7JbtVMwaEspNvH6o1gxubB5Z5pG
''
```