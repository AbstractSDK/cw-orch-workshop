use cw721_minter::deploy::{CosmosAdventures, DeployData};
use cw_orch::{anyhow, daemon::networks::CONSTANTINE_3, prelude::*};

pub fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init();

    let chain = DaemonBuilder::default().chain(CONSTANTINE_3).build()?;

    CosmosAdventures::deploy_on(
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
