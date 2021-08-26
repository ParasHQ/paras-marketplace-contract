const fs = require("fs");
const nearAPI = require("near-api-js");
const getConfig = require("./config");
const { nodeUrl, networkId, contractName, contractMethods } = getConfig('testnet');

const {
	keyStores: { InMemoryKeyStore },
	Near,
	Account,
	Contract,
	KeyPair,
	utils: {
		format: { parseNearAmount },
	},
} = nearAPI;

let credentials;
try {
	credentials = JSON.parse(
		fs.readFileSync(
			`${process.env.HOME}/.near-credentials/${networkId}/${contractName}.json`
		)
	);
} catch(e) {
	console.warn('credentials not found, looking in /neardev');
	credentials = JSON.parse(
		fs.readFileSync(
			`./neardev/${networkId}/${contractName}.json`
		)
	);
}

const keyStore = new InMemoryKeyStore();
keyStore.setKey(
	networkId,
	contractName,
	KeyPair.fromString(credentials.private_key)
);
const near = new Near({
	networkId,
	nodeUrl,
	deps: { keyStore },
});
const { connection } = near;
const contractAccount = new Account(connection, contractName);
contractAccount.addAccessKey = (publicKey) =>
	contractAccount.addKey(
		publicKey,
		contractName,
		contractMethods,
		parseNearAmount("0.1")
	);
const contract = new Contract(contractAccount, contractName, contractMethods);

module.exports = {
	near,
	credentials,
	keyStore,
	connection,
	contract,
	contractName,
	contractAccount,
	contractMethods,
};
