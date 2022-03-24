const marketplaceContractName = "dev-1648023319907-58748753844114";
const nftContractName = "paras-token-v1.testnet";
const ownerAccountName = "testingdo.testnet";

module.exports = function getConfig(network = "testnet") {
  let config = {
    networkId: "testnet",
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
    explorerUrl: "https://explorer.testnet.near.org",
    marketplaceContractName: marketplaceContractName,
    nftContractName: nftContractName,
    ownerAccountName: ownerAccountName,
  };

  switch (network) {
    case "testnet":
      config = {
        ...config,
        GAS: "300000000000000",
        gas: "300000000000000",
        gas_max: "300000000000000",
        DEFAULT_NEW_ACCOUNT_AMOUNT: "2",
        DEFAULT_NEW_CONTRACT_AMOUNT: "5",
        GUESTS_ACCOUNT_SECRET:
          "7UVfzoKZL4WZGF98C3Ue7tmmA6QamHCiB1Wd5pkxVPAc7j6jf3HXz5Y9cR93Y68BfGDtMLQ9Q29Njw5ZtzGhPxv",
      };
  }

  return config;
};
