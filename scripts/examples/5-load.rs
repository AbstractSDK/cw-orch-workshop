use cosmwasm_std::coins;
use cw721_minter::deploy::CwOrchWorkshop;
use cw721_minter::{MinterExecuteMsgFns as _, MinterQueryMsgFns as _};
use cw_orch::daemon::networks::CONSTANTINE_3;
use cw_orch::prelude::*;

// Quest #5.1
// In this quest, you will learn how to load a cw-orch bundle from external crates.
// Here cw721-minter is now a git dependency.
// You should be able with a one-liner to load the bundle directly from this crate.
// The artifacts AND the state are directly loaded from the crate !
// Your task is to replace the `todo!()` directive with actual code to load the crate.
// This should be cheap to run (around 0.3 CONST gas fee + 0.1 CONST NFT minting payment)
pub fn main() -> cw_orch::anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init();
    let chain = DaemonBuilder::default().chain(CONSTANTINE_3).build()?;

    let workshop: CwOrchWorkshop<Daemon> = todo!();

    // We can get the nft address from the contract
    let nft_address = workshop.nft.address()?;
    println!("nft_address is defined here : {}", nft_address);

    // Query the price to pay for an NFT directly on-chain
    let native_price = workshop.minter.state()?.native_price;

    // Mint the NFT
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
