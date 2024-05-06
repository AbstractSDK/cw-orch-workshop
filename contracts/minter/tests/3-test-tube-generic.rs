use cosmwasm_std::coins;
use cw_orch::prelude::*;

// Quest #3.3
// Now that `generic_test` is implemented, make it run on an OmosisTestTube environment!
#[test]
pub fn test_upload_test_tube() -> cw_orch::anyhow::Result<()> {
    // We provide a lot of coins to the sender on this environment to make sure they are able to pay gas fees!
    let mut test_tube = OsmosisTestTube::new(coins(100_000_000_000_000_000, "uosmo"));

    let native_denom = "native_mint";
    test_tube.add_balance(&test_tube.sender(), coins(150_000, native_denom))?;

    // TODO : call the generic_test function with the right arguments to make your test run on the `test_tube` environment.

    Ok(())
}
