use cw721_minter::deploy::{CwOrchWorkshop, DeployData};
use cw_orch::{anyhow, daemon::networks::CONSTANTINE_3, prelude::*};

// This script is costly, you will most likely need around 10 CONST testnet tokens to run it
// Unfortunately the faucet only gives 5.
// If you want to do testnet interactions, look at the 5-load script inside the scripts folder in the root dir
pub fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init();

    let chain = DaemonBuilder::default().chain(CONSTANTINE_3).build()?;

    CwOrchWorkshop::deploy_on(
        chain.clone(),
        DeployData {
            token_name: "cw20-test".to_string(),
            token_symbol: "CWORCH".to_string(),
            token_decimals: 6,
            token_minter: chain.sender().to_string(),
            native_denom: "aconst".to_string(),
            native_price: 100_000u128,
            token_price: 200_000u128,
        },
    )?;

    Ok(())
}
