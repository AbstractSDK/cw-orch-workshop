use cw721_minter::deploy::{CwOrchWorkshop, DeployData};
use cw_orch::prelude::*;

#[test]
pub fn mock_deploy() -> cw_orch::anyhow::Result<()> {
    let mock = MockBech32::new("osmosis");
    let native_denom = "native_mint";

    CwOrchWorkshop::deploy_on(
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

    Ok(())
}
