use cosmwasm_std::coins;
use cw721_minter::interface::generic_test;
use cw_orch::prelude::*;

#[test]
pub fn test_upload_mock_generic() -> cw_orch::anyhow::Result<()> {
    let mock = MockBech32::new("osmosis");

    let native_denom = "native_mint";
    mock.add_balance(&mock.sender(), coins(150_000, native_denom))?;

    generic_test(mock, native_denom.to_string())?;

    Ok(())
}
