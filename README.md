# Cw-orchestrator workshop

This repository contains a workshop to learn [cw-orch](https://orchestrator.abstract.money). There are 5 quests that will help you setup a smart-contract to be able to test, script, deploy and maintain it. Be ready to change the way you work with smart-contracts forever.

All the quests are explained as comment inside the code. They all start with `QUEST #x.y`. If you are looking for a quest, you can search the whole folder for those symbols ! For instance for the first quest, look for `QUEST #1.1`.

## Quest 1

This quest is aimed at introducing the cw-orch setup. You have 3 different quests:

1. Create the Contract interface.
2. Implement the Uploadable trait
3. Apply the macros that facilitate contract interactions

The contract you are touching is a simple counter contract (increment, reset, count), so nothing really interesting here.
But this will allow you to better understand how simple the cw-orch setup is. It's a little boilerplate code, nothing too major.
In case you need help, [the 2-3 first pages](https://orchestrator.abstract.money/contracts/index.html) in this part of the documentation provide all the code and pointer that you need.

## Quest 2

### Presentation

Now let's play with a more interesting contract.
The `minter` or `cw721-minter` contract is a contract that allows users to mint NFTs against a payment of both cw20 and native tokens. To be a little more specific, this contract allows to:

- Mint an NFT by sending native tokens to the `{"mint":{}}` endpoint.
- Mint an NFT by using the `{"send":{}}` endpoint of a CW20 token.

### Specifics

- The native and cw20 token used for minting are whitelisted at contract instantiation.
- The contract is only able to mint 1 NFT per block per address. So if you want to mint an NFT for the same account, you need to wait at least 1 block.

### Sub-quests

Here are the quests that you will encounter in part 2:

1. Register the contract `reply` endpoint.
2. Mint fw20 tokens
3. Mint native tokens
4. Mint an NFT
5. Mint a second NFT

# Quest 3

# Quest 5


To make `5-*` examples work, the following env vars need to be defined : 
- `TEST_MNEMONIC`
- `RUST_LOG`

You can defined them in a `.env` file inside the root directory of this repo