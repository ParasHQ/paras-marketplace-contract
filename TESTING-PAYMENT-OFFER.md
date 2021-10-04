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
near call --accountId dev-1633338079498-71054384021895 dev-1633338079498-71054384021895 new '{"treasury_id":"johnnear.testnet","owner_id":"dev-1633338079498-71054384021895","approved_nft_contract_ids":["dev-1630163823356-82615235483639"]}'
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
near call dev-1633338079498-71054384021895 storage_deposit '{"accountId":"cymac.testnet"}' --accountId cymac.testnet --depositYocto 8590000000000000000000
```
## NFT add offer 
```
near call dev-1633338079498-71054384021895 add_offer '{"nft_contract_id":"dev-1633337760334-37276932331579", "token_id":"1:2", "ft_token_id":"near", "price": "1000000000000000000000000"}' --accountId cymac.testnet --deposit 1
```

## Get offer
```
$ near view dev-1633338079498-71054384021895 get_offer '{"nft_contract_id":"dev-1633337760334-37276932331579","token_id":"1:2","account_id":"cymac.testnet"}'
View call: dev-1633338079498-71054384021895.get_offer({"nft_contract_id":"dev-1633337760334-37276932331579","token_id":"1:2","account_id":"cymac.testnet"})
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
$ near call --accountId gnaor.testnet  dev-1633337760334-37276932331579  nft_approve '{"token_id":"1:12","account_id":"dev-1633338079498-71054384021895","msg":"{\"market_type\":\"accept_offer\",\"account_id\":\"cymac.testnet\"}"}' --depositYocto 440000000000000000000 --gas 300000000000000
Scheduling a call: dev-1633337760334-37276932331579.nft_approve({"token_id":"1:12","account_id":"dev-1633338079498-71054384021895","msg":"{\"market_type\":\"accept_offer\",\"account_id\":\"cymac.testnet\"}"}) with attached 0.00044 NEAR
Receipt: 8yAsQodQDbG7U5VrsYt56K34KNMtycDyA6j9MvSoizgt
Log [dev-1633337760334-37276932331579]: Transfer 1:12 from dev-1633338079498-71054384021895 to cymac.testnet
Log [dev-1633337760334-37276932331579]: {"type":"nft_transfer","params":{"token_id":"1:12","sender_id":"gnaor.testnet","receiver_id":"cymac.testnet"}}
Receipts: AXPqmu3HtUMLMooNahar21JUJzyJ1UxyJRAWHtoSVM9F, 8g7QMhJBYMnk44gmdz1Wok8PSpbbNHXS5rEDSxFnPTQs, 4QMpUz34dThuxKKfGvQ1iVbUxH2EpsZ2rQ3BYmxnyJ8m, J5QrETWsJ4CG3xiueqwEeUoS6vGUoNYGgndU8RfMchqb
Log [dev-1633337760334-37276932331579]: {"type":"resolve_purchase","params":{"owner_id":"gnaor.testnet","nft_contract_id":"dev-1633337760334-37276932331579","token_id":"1:12","ft_token_id":"near","price":"100000000000000000000000","buyer_id":"cymac.testnet"}}
Transaction Id mhfKCh6RxGSwcAohEwMZC2VUv3kteqoGfs5APiGL1aP
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/mhfKCh6RxGSwcAohEwMZC2VUv3kteqoGfs5APiGL1aP
```