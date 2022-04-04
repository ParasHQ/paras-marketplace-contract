const assert = require("assert");
const testUtils = require("./test-utils");

const {
  gas,
  marketplaceContractAccount,
  marketplaceContract,
  marketplaceContractName,
  nftContractName,
  ownerAccount,
  ownerAccountName,
  tokenOwnerAccount,
  bidderAccount,
  bidderAccount2,
  explorerUrl
} = testUtils;

describe("Paras Marketplace Contract", function () {
  this.timeout(6000);

  it("should be deployed", async function () {
    const state = await marketplaceContractAccount.state();
    try {
      await marketplaceContract.new({
        args: {
          owner_id: ownerAccountName,
          treasury_id: marketplaceContractName,
          approved_ft_token_ids: ["near"],
          paras_nft_contract: nftContractName,
          current_fee: 500,
        },
      });
    } catch (err) {
      if (!/contract has already been initialized/.test(err.toString())) {
        console.warn(err);
      }

      assert.notStrictEqual(
        state.code_hash,
        "11111111111111111111111111111111"
      );
    }
  }).timeout(20000);

  it("should add approved nft contract id" , async function() {
    try {
      const result = await ownerAccount.functionCall({
        contractId: marketplaceContractName,
        methodName: 'add_approved_nft_contract_ids',
        args: {
          nft_contract_ids: ["paras-token-v1.testnet"]
        },
        attachedDeposit: "1"
      })

      assert.ok(result.transaction_outcome.id, "Result outcome is not exist")
      console.log(`\n Result approved nft contract id : ${explorerUrl}/transactions/${result.transaction_outcome.id} `)
    } catch (err) {
      console.warn(err)
    }
  }).timeout(20000)

  it("should put token on auction and bid the token", async function () {
    try {
      // Storage Deposit on Marketplace Contract
      await tokenOwnerAccount.functionCall({
        contractId: marketplaceContractName,
        methodName: "storage_deposit",
        args: {
          account_id: tokenOwnerAccount.accountId,
        },
        gas: gas,
        attachedDeposit: "8590000000000000000000",
      });

      // NFT Approve on NFT Contract
      const ended_at = `${(Date.now() + 4 * 60 * 60 * 1000).toString()}000000`

      const putTokenOnAuction = await tokenOwnerAccount.functionCall({
        contractId: nftContractName,
        methodName: "nft_approve",
        args: {
          token_id: "507:1",
          account_id: marketplaceContractName,
          msg: JSON.stringify({
            price: "10000000000000000000000000",
            ft_token_id: 'near',
            market_type: 'sale',
            started_at: `${(Date.now() + 60 * 60).toString()}000000`,
            ended_at: ended_at,
            is_auction: true
          })
        },
        attachedDeposit: "440000000000000000000",
      });
      assert.notEqual(putTokenOnAuction.type, 'FunctionCallError')

      const market_data_before_bid = await marketplaceContractAccount.viewFunction(
        marketplaceContractName,
        "get_market_data",
        {
          nft_contract_id: nftContractName,
          token_id: "507:1",
        },
      );
      console.log("Market Data before bid : \n" , market_data_before_bid)

      assert.ok(market_data_before_bid !== null);

      // Add Bid
      const bid = await bidderAccount.functionCall({
        contractId: marketplaceContractName,
        methodName: "add_bid",
        args: {
          nft_contract_id: nftContractName,
          ft_token_id: "near",
          token_id: "507:1",
          amount: "10500000000000000000000000",
        },
        attachedDeposit: "10500000000000000000000000",
      });

      const market_data_after_bid = await marketplaceContractAccount.viewFunction(
        marketplaceContractName,
        "get_market_data",
        {
          nft_contract_id: nftContractName,
          token_id: "507:1",
        },
      );
      console.log("Market Data after bid : \n" , market_data_after_bid)

      // Add Bid 2
      const bid2 = await bidderAccount2.functionCall({
        contractId: marketplaceContractName,
        methodName: "add_bid",
        args: {
          nft_contract_id: nftContractName,
          ft_token_id: "near",
          token_id: "507:1",
          amount: "11500000000000000000000000",
        },
        attachedDeposit: "11500000000000000000000000",
      });

      const market_data_after_bid2 = await marketplaceContractAccount.viewFunction(
        marketplaceContractName,
        "get_market_data",
        {
          nft_contract_id: nftContractName,
          token_id: "507:1",
        },
      );
      console.log("Market Data after bid 2 : \n" , market_data_after_bid2)

      // Add Bid 3
      const bid3 = await bidderAccount.functionCall({
        contractId: marketplaceContractName,
        methodName: "add_bid",
        args: {
          nft_contract_id: nftContractName,
          ft_token_id: "near",
          token_id: "507:1",
          amount: "12500000000000000000000000",
        },
        attachedDeposit: "12500000000000000000000000",
      });

      const market_data_after_bid3 = await marketplaceContractAccount.viewFunction(
        marketplaceContractName,
        "get_market_data",
        {
          nft_contract_id: nftContractName,
          token_id: "507:1",
        },
      );
      console.log("Market Data after bid 3 : \n" , market_data_after_bid3)

      // Accept Bid
      await tokenOwnerAccount.functionCall({
        contractId: marketplaceContractName,
        methodName: "accept_bid",
        args: {
          nft_contract_id: nftContractName,
          token_id: "507:1",
        },
        attachedDeposit: "1",
        gas: gas,
      });

      const newOwner = await tokenOwnerAccount.viewFunction(
        nftContractName,
        "nft_token",
        {
          token_id: "507:1",
        },
      );

      console.log("Token Info after accept bid: " , newOwner)
      assert.equal(newOwner.owner_id, bidderAccount.accountId);
    } catch (err) {
      console.warn(err);
    }
  }).timeout(40000);

  it("should put nft on sale", async function() {
    try {
      const result = await bidderAccount.functionCall({
        contractId: nftContractName,
        methodName: 'nft_approve',
        args: {
          token_id: '507:1',
          account_id: marketplaceContractName,
          msg: JSON.stringify({
            price: '10000000000000000000000000',
            ft_token_id: 'near',
            market_type: 'sale'
          })
        },
        attachedDeposit: '440000000000000000000'
      })

      assert.ok(result.transaction_outcome.id, "Result outcome is not exist")
      console.log(`\n Result put nft on sale : ${explorerUrl}/transactions/${result.transaction_outcome.id}`)
    } catch (err) { 
      console.warn(err)
    }
  }).timeout(20000)

  it("should buy nft", async function() {
    try {
    const result = await tokenOwnerAccount.functionCall({
      contractId: marketplaceContractName,
      methodName: "buy",
      args: {
        nft_contract_id: nftContractName,
        token_id: "507:1",
        ft_token_id: "near",
        price: "10000000000000000000000000"
      },
      gas: gas,
      attachedDeposit: "10000000000000000000000000"
    })

    assert.ok(result.transaction_outcome.id, "Result outcome is not exist")
    console.log(`\n Result buy : ${explorerUrl}/transactions/${result.transaction_outcome.id}`)
    } catch (err) {
      console.warn(err)
    }
  }).timeout(20000)
});
