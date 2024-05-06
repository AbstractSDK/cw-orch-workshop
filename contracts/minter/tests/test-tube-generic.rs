use cosmwasm_std::coins;
use cw721_minter::interface::generic_test;
use cw_orch::prelude::*;

#[test]
pub fn test_upload_test_tube() -> cw_orch::anyhow::Result<()> {
    let mut test_tube = OsmosisTestTube::new(coins(100_000_000_000_000_000, "uosmo"));

    let native_denom = "native_mint";
    test_tube.add_balance(&test_tube.sender(), coins(150_000, native_denom))?;

    generic_test(test_tube, native_denom.to_string())?;

    Ok(())
}
