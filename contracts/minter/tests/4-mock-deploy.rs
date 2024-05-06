use cw721_minter::{
    deploy::{CwOrchWorkshop, DeployData},
    MinterQueryMsgFns,
};
use cw_orch::prelude::*;

// Quest #4.2.1
// This is not a quest per-say, but you can test your 4.1 implementation here inside a test environment!
#[test]
pub fn mock_deploy() -> cw_orch::anyhow::Result<()> {
    let mock = MockBech32::new("osmosis");
    let native_denom = "native_mint";

    let bundle = CwOrchWorkshop::deploy_on(
        mock.clone(),
        DeployData {
            token_name: "cw20-test".to_string(),
            token_symbol: "CWORCH".to_string(),
            token_decimals: 6,
            token_minter: mock.sender().to_string(),
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
