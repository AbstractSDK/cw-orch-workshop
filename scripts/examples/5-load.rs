use cw721_minter::deploy::CwOrchWorkshop;
use cw_orch::daemon::networks::CONSTANTINE_3;
use cw_orch::prelude::*;

pub fn main() -> cw_orch::anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init();
    let chain = DaemonBuilder::default().chain(CONSTANTINE_3).build()?;

    let workshop = CwOrchWorkshop::load_from(chain)?;

    // We can get the nft address from the contract
    let nft_address = workshop.nft.address()?;

    println!("nft_address is defined here : {}", nft_address);

    Ok(())
}
