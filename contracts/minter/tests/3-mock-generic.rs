use cosmwasm_std::coins;
use cw_orch::prelude::*;

// Quest #3.2
// Now that `generic_test` is implemented, make it run on a MockBech32 environment!
#[test]
pub fn test_upload_mock_generic() -> cw_orch::anyhow::Result<()> {
    let mock = MockBech32::new("osmosis");

    let native_denom = "native_mint";
    mock.add_balance(&mock.sender(), coins(150_000, native_denom))?;

    // TODO : call the generic_test function with the right arguments to make your test run on the `mock` environment.

    Ok(())
}
