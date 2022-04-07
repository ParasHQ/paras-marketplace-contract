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
      };
  }

  return config;
};
