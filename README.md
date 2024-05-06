# Cw-orchestrator workshop

This repository contains a workshop to learn [cw-orch](https://orchestrator.abstract.money). There are 5 quests that will help you setup a smart-contract to be able to test, script, deploy and maintain it. Be ready to change the way you work with smart-contracts forever.

All the quests are explained as comment inside the code. They all start with `QUEST #x.y`. If you are looking for a quest, you can search the whole folder for those symbols ! For instance for the first quest, look for `QUEST #1.1`.

The solutions are located inside the `solution` branch of this repository. [You can access them online](https://github.com/AbstractSDK/cw-orch-workshop/tree/solution), or by running `git checkout solution` inside this repo.

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

In this quest, you will learn how to test your contract inside a local Rust environment. You will see what cw-orch provides and how easy it is to test your contracts! All the code in this quest is actually ran inside [`cw-multi-test`](https://github.com/CosmWasm/cw-multi-test/).

### Specifics

- The native and cw20 token used for minting are whitelisted at contract instantiation.
- The contract is only able to mint 1 NFT per block per address. So if you want to mint an NFT for the same account, you need to wait at least 1 block.

### Sub-quests

Here are the quests that you will encounter in part 2:

1. Register the contract `reply` endpoint.
2. Mint cw20 tokens
3. Mint native tokens
4. Mint an NFT
5. Mint a second NFT

## Quest 3

Now we have seen how to test your contracts with a very straightforward and simple syntax inside cw-multi-test. Cw-orch however provides multiple other execution environments:

- Test Tube: executing locally on an actual chain binaries, without leaving your code.
- Daemon: executing on actual block-chains.
- .. Learn more about execution environments in the [official cw-orch documentation](https://orchestrator.abstract.money/).

In this quest, we will make the code generic to be able to use it on any chain supported by cw-orch. Let's dive in:

1. Create a generic function that you can call with any CwEnv environment.
2. Call the function on a `MockBech32` environment
3. Call the function on a `OsmosisTestTube` environment


## Quest 5


To make `5-*` examples work, the following env vars need to be defined : 
- `TEST_MNEMONIC`
- `RUST_LOG`

You can defined them in a `.env` file inside the root directory of this repo