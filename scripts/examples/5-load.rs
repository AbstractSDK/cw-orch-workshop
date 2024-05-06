use cosmwasm_std::coins;
use cw721_minter::deploy::CwOrchWorkshop;
use cw721_minter::{MinterExecuteMsgFns as _, MinterQueryMsgFns as _};
use cw_orch::daemon::networks::CONSTANTINE_3;
use cw_orch::prelude::*;

// This is cheaper to run (around 0.3 CONST gas fee + 0.1 CONST NFT minting payment)
pub fn main() -> cw_orch::anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init();
    let chain = DaemonBuilder::default().chain(CONSTANTINE_3).build()?;

    let workshop = CwOrchWorkshop::load_from(chain.clone())?;

    // We can get the nft address from the contract
    let nft_address = workshop.nft.address()?;
    println!("nft_address is defined here : {}", nft_address);

    let native_price = workshop.minter.state()?.native_price;

    workshop
        .minter
        .mint(&coins(native_price.u128(), CONSTANTINE_3.gas_denom))?;

    // We print all the nfts that belong to you :
    let all_tokens: cw721::TokensResponse = workshop.nft.query(&cw721_base::msg::QueryMsg::<
        Empty,
    >::Tokens {
        owner: chain.sender().to_string(),
        start_after: None,
        limit: None,
    })?;

    println!(
        "All tokens belonging to this address are : {:?}",
        all_tokens
    );

    Ok(())
}
