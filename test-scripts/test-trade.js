const Near = require('./helpers/Near')
const { transactions } = require('near-api-js')

const nftContractId = ''
const marketplaceContractid = ''

const getEventLogFromContractResult = (contractResult) => {
        let logs
        contractResult.receipts_outcome.forEach((v) => {
                if (v.outcome.logs.length !== 0) {
                        logs = v.outcome.logs
                }
        })
        const result = (logs[0].split("EVENT_JSON:")[1]);
        return JSON.parse(result)
}

const nftBuy = async (account, accountReceiver) => {
        const result = await account.functionCall({
                contractId: nftContractId,
                methodName: "nft_buy",
                args: { "token_series_id": "1", "receiver_id": accountReceiver },
                gas: "300000000000000",
                attachedDeposit: '1035000000000000000000000',
        })
        return result
}

const nftAddTrade = async (account, buyerTokenId, sellerTokenId) => {
        const result = await account.functionCall({
                contractId: nftContractId,
                methodName: "nft_approve",
                args: { "token_id": buyerTokenId, "account_id": marketplaceContractid, "msg": `{\"market_type\":\"add_trade\",\"seller_nft_contract_id\":\"${nftContractId}\",\"seller_token_id\":\"${sellerTokenId}\"}` },
                gas: "300000000000000",
                attachedDeposit: '1035000000000000000000000',
        })
        return result
}

const nftAcceptTrade = async (account, buyerTokenId, buyerAccount, sellerTokenId) => {
        const result = await account.functionCall({
                contractId: nftContractId,
                methodName: "nft_approve",
                args: { "token_id": sellerTokenId, "account_id": marketplaceContractid, "msg": `{\"market_type\":\"accept_trade\",\"buyer_id\":\"${buyerAccount}\",\"buyer_nft_contract_id\":\"${nftContractId}\",\"buyer_token_id\":\"${buyerTokenId}\"}` },
                gas: "300000000000000",
                attachedDeposit: '1035000000000000000000000',
        })
        return result

}

const nftTransfer = async (account, tokenId, accountReciver) => {
        const result = await account.functionCall({
                contractId: nftContractId,
                methodName: "nft_transfer",
                args: { "token_id": tokenId, "receiver_id": accountReciver },
                gas: "300000000000000",
                attachedDeposit: '1',
        })
        return result
}

const nftAcceptTransferBatch = async (account, buyerTokenId, buyerAccount, sellerTokenId, tokenId, accountReciver) => {
        const actions = [
                transactions.functionCall(
                        "nft_approve",
                        { "token_id": sellerTokenId, "account_id": marketplaceContractid, "msg": `{\"market_type\":\"accept_trade\",\"buyer_id\":\"${buyerAccount}\",\"buyer_nft_contract_id\":\"${nftContractId}\",\"buyer_token_id\":\"${buyerTokenId}\"}` },
                        "100000000000000",
                        "9870000000000000000000"
                ),
                transactions.functionCall(
                        "nft_transfer",
                        { "token_id": tokenId, "receiver_id": accountReciver },
                        "100000000000000",
                        "1"
                )
        ]

        const result = await account.signAndSendTransaction({
                receiverId: nftContractId,
                actions
        })

        return result
}

const process = async () => {
        const dataAccounts = [
                { "account_id": "trader-a.testnet", "public_key": "ed25519:D4FKUSm255iJYsD2af5VtRLJej1Qe14yoKQkgTyBg2z1", "private_key": "ed25519:37KiZBwHJGchjgydZBaQrNFFA48kfKKvZV8BQcMNqLaHZsoo7FXiQGZgrfk2xvySZW5CDB4h1j9G4b8ofX6M7LB7" },
                { "account_id": "trader-b.testnet", "public_key": "ed25519:FGL5TTpjTqHnoT1SCP5gR5xpDQh6hNzJVLDKgSFwuJrt", "private_key": "ed25519:2ZXMSjV3P8bRLJcmRD94U7xhx69rLQwyM7kfmiQa6JgLWsinawtWepupdiTQdFf9ZTosVTwhHAk7MEU5ug2pzyxc" }
        ]

        const nearDev = new Near(dataAccounts)
        await nearDev.init()

        const traderA = nearDev.account(0)
        const traderB = nearDev.account(1)

        //nft buy
        const nftBuyARresult = getEventLogFromContractResult(await nftBuy(traderA, dataAccounts[0].account_id))
        const nftBuyBRresult = getEventLogFromContractResult(await nftBuy(traderB, dataAccounts[1].account_id))

        const tokenIdA = nftBuyARresult.data[0].token_ids[0]
        const tokenIdB = nftBuyBRresult.data[0].token_ids[0]

        //init
        console.log("init token trader A: ", tokenIdA)
        console.log("init token trader B: ", tokenIdB)

        //traderA trade nft
        await nftAddTrade(traderA, tokenIdA, tokenIdB)

        //traderB accept nft
        // await Promise.all([
        //         nftAcceptTrade(traderB, tokenIdA, dataAccounts[0].account_id, tokenIdB),
        //         nftTransfer(traderB, tokenIdB, 'testnet')
        // ])
        await nftAcceptTransferBatch(traderB, tokenIdA, dataAccounts[0].account_id, tokenIdB, tokenIdB, 'testnet')

}

const main = async () => {
        await process()
}

main()
