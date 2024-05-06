use cosmwasm_std::coins;
use cw721_minter::{
    deploy::{CwOrchWorkshop, DeployData},
    MinterQueryMsgFns,
};
use cw_orch::prelude::*;

#[test]
pub fn test_tube_deploy() -> cw_orch::anyhow::Result<()> {
    let test_tube = OsmosisTestTube::new(coins(100_000_000_000_000_000, "uosmo"));
    let native_denom = "uosmo";

    let bundle = CwOrchWorkshop::deploy_on(
        test_tube.clone(),
        DeployData {
            token_name: "cw20-test".to_string(),
            token_symbol: "CWORCH".to_string(),
            token_decimals: 6,
            token_minter: test_tube.sender().to_string(),
            native_denom: native_denom.to_string(),
            native_price: 100_000u128,
            token_price: 200_000u128,
        },
    )?;

    assert_eq!(
        bundle.nft.address()?.to_string(),
        bundle.minter.state()?.nft_address
    );
    assert_eq!(
        bundle.token.address()?.to_string(),
        bundle.minter.state()?.cw20_address
    );

    Ok(())
}
