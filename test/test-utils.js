const nearAPI = require("near-api-js");
const {
  utils: {
    format: { parseNearAmount },
  },
} = nearAPI;
const getConfig = require("./config");
const {
  networkId,
  marketplaceContractName,
  nftContractName,
  ownerAccountName,
  contractMethods,
  gas,
  gas_max,
  nodeUrl,
  walletUrl,
  explorerUrl,
} = getConfig("testnet");

const keyStore = new nearAPI.keyStores.UnencryptedFileSystemKeyStore(
  `${process.env.HOME}/.near-credentials/`
);

const near = new nearAPI.Near({
  deps: {
    keyStore: keyStore,
  },
  networkId: networkId,
  keyStore: keyStore,
  nodeUrl: nodeUrl,
  walletUrl: walletUrl,
});

const marketplaceContractAccount = new nearAPI.Account(
  near.connection,
  marketplaceContractName
);
const ownerAccount = new nearAPI.Account(near.connection, ownerAccountName);

// Use your own account that logged in on NEAR CLI
const tokenOwnerAccount = new nearAPI.Account(near.connection, "castleoverlord.testnet");
const bidderAccount = new nearAPI.Account(
  near.connection,
  "bobol.testnet"
);
const bidderAccount2 = new nearAPI.Account(
  near.connection,
  "nearmonster.testnet"
);

marketplaceContractAccount.addAccessKey = (publicKey) =>
  marketplaceContractAccount.addKey(
    publicKey,
    marketplaceContractName,
    {
      viewMethods: [
        "get_market_data",
        "get_offer",
        "approved_ft_token_ids",
        "approved_nft_contract_ids",
        "get_owner",
        "get_treasury",
        "get_supply_by_owner_id",
        "storage_minimum_balance",
        "storage_balance_of",
      ],
      changeMethods: [
        "new",
        "storage_deposit",
        "storage_withdraw",
        "add_approved_ft_token_ids",
        "add_approved_nft_contract_ids",
        "add_approved_paras_nft_contract_ids",
        "remove_approved_nft_contract_ids",
        "buy",
        "add_offer",
        "delete_offer",
        "add_bid",
        "accept_bid",
        "update_market_data",
        "delete_market_data",
      ],
    },
    parseNearAmount("0.1")
  );

const marketplaceContract = new nearAPI.Contract(
  marketplaceContractAccount,
  marketplaceContractName,
  {
    viewMethods: [
      "get_market_data",
      "get_offer",
      "approved_ft_token_ids",
      "approved_nft_contract_ids",
      "get_owner",
      "get_treasury",
      "get_supply_by_owner_id",
      "storage_minimum_balance",
      "storage_balance_of",
    ],
    changeMethods: [
      "new",
      "storage_deposit",
      "storage_withdraw",
      "add_approved_ft_token_ids",
      "add_approved_nft_contract_ids",
      "add_approved_paras_nft_contract_ids",
      "remove_approved_nft_contract_ids",
      "buy",
      "add_offer",
      "delete_offer",
      "add_bid",
      "accept_bid",
      "update_market_data",
      "delete_market_data",
    ],
  }
);

const nftContract = new nearAPI.Contract(ownerAccount, nftContractName, {
  viewMethods: ["nft_token"],
  changeMethods: ["nft_transfer", "nft_transfer_call"],
});

module.exports = {
  near,
  gas,
  gas_max,
  keyStore,
  marketplaceContractAccount,
  marketplaceContractName,
  nftContractName,
  ownerAccountName,
  marketplaceContract,
  nftContract,
  ownerAccount,
  contractMethods,
  tokenOwnerAccount,
  bidderAccount,
  bidderAccount2,
  explorerUrl
};
