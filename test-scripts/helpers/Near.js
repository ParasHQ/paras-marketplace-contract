const { connect, Contract, KeyPair, keyStores } = require('near-api-js')

class Near {
        constructor(accounts) {
                this.accounts = accounts

                this.nearConnect = null
                this.nearAccounts = {}
        }

        async init() {
                const keyStore = new keyStores.InMemoryKeyStore()
                let accountPromises = this.accounts.map(async (account) => {
                        const keyPair = KeyPair.fromString(account.private_key)
                        return await keyStore.setKey('testnet', account.account_id, keyPair);
                })

                await Promise.all(accountPromises)

                const nearConfig = {
                        deps: {
                                keyStore
                        },
                        networkId: 'testnet',
                        nodeUrl: 'https://rpc.testnet.near.org',
                        walletUrl: 'https://wallet.testnet.near.org',
                        helperUrl: 'https://helper.testnet.near.org',
                }

                this.nearConnect = await connect(nearConfig)

                let nearAccountPromises = Object.keys(this.accounts).map(async (accountName) => {
                        const account = this.accounts[accountName]
                        this.nearAccounts[accountName] = await this.nearConnect.account(account.account_id)
                })
                await Promise.all(nearAccountPromises)
        }

        account(accountName) {
                return this.nearAccounts[accountName]
        }
}

module.exports = Near

