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
    cw20.mint(150_000u128.into(), mock.sender().to_string())?;

    let native_denom = "native_mint";
    mock.add_balance(&mock.sender(), coins(150_000, native_denom))?;

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

    minter.mint(&coins(200, native_denom))?;

    let minted: TokensResponse = cw721.query(&cw721_base::QueryMsg::AllTokens {
        start_after: None,
        limit: None,
    })?;
    assert_eq!(minted.tokens.len(), 1);

    // We mint another NFT but it needs to advance blocks to be able to mint
    mock.wait_blocks(1)?;
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
