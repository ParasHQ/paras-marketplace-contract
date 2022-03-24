const nearAPI = require("near-api-js");
const {
  KeyPair,
  Account,
  Contract,
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
  DEFAULT_NEW_ACCOUNT_AMOUNT,
  DEFAULT_NEW_CONTRACT_AMOUNT,
  GUESTS_ACCOUNT_SECRET,
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
const tokenOwnerAccount = new nearAPI.Account(near.connection, "castleoverlord.testnet");
const bidderAccount = new nearAPI.Account(
  near.connection,
  "bobol.testnet"
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

async function initContract() {
  try {
    await marketplaceContract.new({
      owner_id: marketplaceContractName,
    });
  } catch (e) {
    throw e;
  }
  return { marketplaceContract, marketplaceContractName };
}

const getAccountBalance = async (accountId) =>
  new nearAPI.Account(connection, accountId).getAccountBalance();

const initAccount = async (accountId, secret) => {
  account = new nearAPI.Account(connection, accountId);
  const newKeyPair = KeyPair.fromString(secret);
  keyStore.setKey(networkId, accountId, newKeyPair);
  return account;
};

const createOrInitAccount = async (
  accountId,
  secret = GUESTS_ACCOUNT_SECRET,
  amount = DEFAULT_NEW_CONTRACT_AMOUNT
) => {
  let account;
  try {
    account = await createAccount(accountId, amount, secret);
  } catch (e) {
    if (!/because it already exists/.test(e.toString())) {
      throw e;
    }
    account = initAccount(accountId, secret);
  }
  return account;
};

async function getAccount(
  accountId,
  fundingAmount = DEFAULT_NEW_ACCOUNT_AMOUNT,
  secret
) {
  accountId = accountId || generateUniqueSubAccount();
  const account = new nearAPI.Account(connection, accountId);
  try {
    await account.state();
    return account;
  } catch (e) {
    if (!/does not exist/.test(e.toString())) {
      throw e;
    }
  }
  return await createAccount(accountId, fundingAmount, secret);
}

async function getContract(account) {
  return new Contract(account || contractAccount, contractName, {
    ...contractMethods,
    signer: account || undefined,
  });
}

const createAccessKeyAccount = (key) => {
  connection.signer.keyStore.setKey(networkId, contractName, key);
  return new Account(connection, contractName);
};

function generateUniqueSubAccount() {
  return `t${Date.now()}.${contractName}`;
}

/// internal
async function createAccount(
  accountId,
  fundingAmount = DEFAULT_NEW_ACCOUNT_AMOUNT,
  secret
) {
  const contractAccount = new Account(connection, contractName);
  const newKeyPair = secret
    ? KeyPair.fromString(secret)
    : KeyPair.fromRandom("ed25519");
  await contractAccount.createAccount(
    accountId,
    newKeyPair.publicKey,
    new BN(parseNearAmount(fundingAmount))
  );
  keyStore.setKey(networkId, accountId, newKeyPair);
  return new nearAPI.Account(connection, accountId);
}

module.exports = {
  near,
  gas,
  gas_max,
  keyStore,
  getContract,
  getAccountBalance,
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
  initAccount,
  createOrInitAccount,
  createAccessKeyAccount,
  initContract,
  getAccount,
  explorerUrl
};
