const assert = require('assert');
const { parseNearAmount } = require('near-api-js/lib/utils/format');
const { contract } = require('./near-utils');
const testUtils = require('./test-utils');

const {
	gas,
	contractId,
	contractAccount,
	getAccount,
} = testUtils;

describe('NFT Series', function () {
	this.timeout(10000);

	const now = Date.now().toString()
	let token_type = 'dog-' + now

	/// users
	const aliceId = 'alice-' + now + '.' + contractId;
	let alice
	it('should create user accounts', async function () {
		alice = await getAccount(aliceId);
		console.log('\n\n alice accountId:', aliceId, '\n\n');
	})

	it('should be deployed', async function () {
		const state = await contractAccount.state()
		try {
			await contractAccount.functionCall({
				contractId,
				methodName: 'new_default_meta',
				args: {
					owner_id: contractId
				},
				gas
			})
		} catch (e) {
			if (!/contract has already been initialized/.test(e.toString())) {
				console.warn(e)
			}
		}

		assert.notStrictEqual(state.code_hash, '11111111111111111111111111111111');
	});

	it('should allow owner to create a type', async function () {
		await contractAccount.functionCall({
			contractId,
			methodName: 'nft_create_type',
			args: {
				token_metadata: {
					title: token_type,
					media: 'https://placedog.net/500',
					copies: 1,
				},
				token_type: token_type,
				price: "1000000000000000000000000"
			},
			gas,
			attachedDeposit: parseNearAmount('0.1')
		})

		const [token_type_ret, owner_id, type_metadata] = await contractAccount.viewFunction(
			contractId,
			'nft_get_type_info',
			{
				token_type
			}
		)

		assert.strictEqual(token_type_ret, token_type);
		assert.strictEqual(owner_id, contractId);
		assert.strictEqual(type_metadata.copies, 1);

		const types = await contractAccount.viewFunction(
			contractId,
			'nft_get_types',
			{
				limit: 10
			}
		)

		console.log(types)
		assert.strictEqual(types.length, 1);
	});

	it('should NOT allow non-owner to create type', async function() {
		try {
			await alice.functionCall({
				contractId,
				methodName: 'nft_create_type',
				args: {
				token_metadata: {
					title: token_type,
					media: 'https://placedog.net/500',
					copies: 1,
				},
				token_type: token_type,
				price: "1000000000000000000000000"
			},
			gas,
			attachedDeposit: parseNearAmount('0.1')
			})
			assert(false)
		} catch(e) {
			assert(true)
		}
	});

	it('should NOT create the same type', async function () {
		try {
			await contractAccount.functionCall({
				contractId,
				methodName: 'nft_create_type',
				args: {
				token_metadata: {
					title: token_type,
					media: 'https://placedog.net/500',
					copies: 1,
				},
				token_type: token_type,
				price: "1000000000000000000000000"
			},
			gas,
			attachedDeposit: parseNearAmount('0.1')
			})
			assert(false)
		} catch(e) {
			assert(true)
		}

	});

	it('should NOT allow a NON owner to mint copies', async function () {
		try {
			await alice.functionCall({
				contractId,
				methodName: 'nft_mint_type',
				args: {
					token_type,
					receiver_id: contractId
				},
				gas,
				attachedDeposit: parseNearAmount('0.1')
			})
			assert(false)
		} catch(e) {
			assert(true)
		}
	});

	it('should allow the owner to mint a token of a particular type', async function () {

		// const stateBefore = await (await getAccount(contractId)).state();
		// console.log('stateBefore', stateBefore)

		await contractAccount.functionCall({
			contractId,
			methodName: 'nft_mint_type',
			args: {
				token_type,
				receiver_id: contractId
			},
			gas,
			attachedDeposit: parseNearAmount('0.1')
		})

		// const stateAfter = await (await getAccount(contractId)).state();
		// console.log('stateAfter', stateAfter)

		const supply_for_type = await contractAccount.viewFunction(
			contractId,
			'nft_supply_for_type',
			{
				token_type
			}
		)
		assert.strictEqual(parseInt(supply_for_type, 10), 1);

		const tokens = await contractAccount.viewFunction(
			contractId,
			'nft_tokens_by_type',
			{
				token_type
			}
		)
		const [TOKEN_DELIMETER, TITLE_DELIMETER, EDITION_DELIMETER] = await contractAccount.viewFunction(
			contractId,
			'nft_get_type_format',
		)
		const { token_id, owner_id, metadata: { title, copies } } = tokens[0]

		assert.strictEqual(token_id, token_type + ':1');
		assert.strictEqual(title, token_type);
		assert.strictEqual(owner_id, contractId);
	});

	it('should allow anyone to buy', async function() {
		await alice.functionCall({
			contractId,
			methodName: 'nft_buy',
			args: {
				token_type: token_type,
				receiver_id: aliceId
			},
			gas,
			attachedDeposit: parseNearAmount('1')
		});

		const tokens = await contractAccount.viewFunction(
			contractId,
			'nft_tokens_by_type',
			{
				token_type
			}
		)

		const { token_id, owner_id, metadata: { title, copies } } = tokens[1]

		const token_type_balance = await contractAccount.viewFunction(
			contractId,
			'nft_type_balance',
			{
				account_id: aliceId,
				token_type: token_type
			}
		);

		assert.strictEqual(token_id, token_type + ":2");
		assert.strictEqual(token_type_balance, "1");
	});

	it('should NOT allow mint after set mintable to false', async function () {
		await contractAccount.functionCall({
			contractId,
			methodName: 'nft_set_type_mintable',
			args: {
				token_type: token_type,
				is_mintable: false
			},
			gas,
			attachedDeposit: "1"
		});

		try {
			await contractAccount.functionCall({
				contractId,
				methodName: 'nft_mint_type',
				args: {
					token_type,
					receiver_id: contractId
				},
				gas,
				attachedDeposit: parseNearAmount('0.1')
			})
			assert(false)
		} catch(e) {
			assert(true)
		}
	});

	it('should NOT allow the owner to mint more than copies', async function () {
		try {
			await contractAccount.functionCall({
				contractId,
				methodName: 'nft_mint_type',
				args: {
					token_type,
					receiver_id: contractId
				},
				gas,
				attachedDeposit: parseNearAmount('0.1')
			})
			assert(false)
		} catch(e) {
			assert(true)
		}
	});
})
