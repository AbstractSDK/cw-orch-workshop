use cosmwasm_std::{coins, to_json_binary};
use cw20::msg::Cw20ExecuteMsgFns;
use cw20::MinterResponse;
use cw721::TokensResponse;
use cw721_base::interface::Cw721;
use cw721_minter::{
    msg::{ExecuteMsg, InstantiateMsg},
    MinterContract, MinterExecuteMsgFns, MinterQueryMsgFns,
};
use cw_orch::prelude::*;
use cw_plus_interface::cw20_base::{self, Cw20Base};

#[test]
pub fn test_upload() -> cw_orch::anyhow::Result<()> {
    let mock = MockBech32::new("osmosis");
    let cw721 = Cw721::new("nft", mock.clone());
    cw721.upload()?;

    let cw20 = Cw20Base::new("cw20", mock.clone());
    cw20.upload()?;
    cw20.instantiate(
        &cw20_base::InstantiateMsg {
            name: "cw20-test".to_string(),
            symbol: "CWORCH".to_string(),
            decimals: 6,
            initial_balances: vec![],
            mint: Some(MinterResponse {
                minter: mock.sender().to_string(),
                cap: None,
            }),
            marketing: None,
        },
        None,
        None,
    )?;

    // QUEST #2.2
    // Mint some cw20 tokens to your address. These are needed to be able to mint an NFT
    // Tips :
    // - Consider importing the `use cw20::msg::Cw20ExecuteMsgFns` trait
    // - This link can help your as well : https://orchestrator.abstract.money/integrations/daemon.html#interacting-with-contracts

    let native_denom = "native_mint";
    // QUEST #2.3
    // Mint some native tokens with the `native_denom` denom to your address.
    // These are another way to mint an NFT in the cw721_minter contract
    // Tips :
    // - Consider exploring the documentation of the MockBech32 struct : https://docs.rs/cw-orch-mock/latest/cw_orch_mock/type.MockBech32.html

    let minter = MinterContract::new(mock.clone());
    minter.upload()?;
    minter.instantiate(
        &InstantiateMsg {
            native_denom: native_denom.to_string(),
            native_price: 200u128.into(),
            cw20_address: cw20.address()?.to_string(),
            cw20_price: 1500u128.into(),
            nft_code_id: cw721.code_id()?,
        },
        None,
        None,
    )?;

    let state = minter.state()?;
    cw721.set_address(&Addr::unchecked(state.nft_address));

    // QUEST #2.4
    // Mint an NFT using native funds !
    // Tips :
    // - No tips, you should be able to do this on your own now !

    let minted: TokensResponse = cw721.query(&cw721_base::QueryMsg::AllTokens {
        start_after: None,
        limit: None,
    })?;
    assert_eq!(minted.tokens.len(), 1);

    // QUEST #2.5
    // We want to mint an NFT using cw20 tokens in this line.
    // However, as explained in the introduction to those quests (see README.md), the contract can only mint on NFT per block per address
    // So you need a way to change the current block that the blockchain is on !
    // Tips :
    // - No tips, you should be able to do this on your own now ! Documentation (docs.rs and orchestrator.abstract.money are your best friends here)
    // Once you are done, this test should run ! (`cargo test --test 2-mock`)
    cw20.send(
        1500u128.into(),
        minter.address()?.to_string(),
        to_json_binary(&ExecuteMsg::Mint {})?,
    )?;

    let minted: TokensResponse = cw721.query(&cw721_base::QueryMsg::AllTokens {
        start_after: None,
        limit: None,
    })?;
    assert_eq!(minted.tokens.len(), 2);

    Ok(())
}
