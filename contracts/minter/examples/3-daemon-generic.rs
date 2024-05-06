use cw_orch::{anyhow, daemon::networks::CONSTANTINE_3, prelude::*};

// Quest #3.4
// Now that `generic_test` is implemented, make it run on an actual testnet!
// In order to do that, you will have to define the `TEST_MNEMONIC` environment variable with a valid mnemonic and some funds on the corresponding address
// (NEVER USE MAINNET MNEMONICS ON TESTNET)
// This script is costly, you will most likely need around 10 CONST testnet tokens to run it
// You may want to try this on another testnet network (try juno testnet `networks::UNI_6` for instance)
// Look a `.env.example` for other env variables you can enable to follow the execution in your console
// Create a `.env` file with the keys from `.env.example` for a better experience
pub fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init();

    let chain = DaemonBuilder::default().chain(CONSTANTINE_3).build()?;

    // TODO : call the generic_test function with the right arguments to make your test run on an actual testnet !

    Ok(())
}
