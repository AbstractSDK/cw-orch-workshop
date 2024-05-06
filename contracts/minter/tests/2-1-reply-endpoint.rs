use cw20::MinterResponse;
use cw721_base::interface::Cw721;
use cw721_minter::msg::InstantiateMsg;
use cw_orch::prelude::*;
use cw_plus_interface::cw20_base::{self, Cw20Base};

#[test]
pub fn test_upload() -> cw_orch::anyhow::Result<()> {
    let mock = MockBech32::new("osmosis");
    let native_denom = "native_mint";

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

    Ok(())
}
