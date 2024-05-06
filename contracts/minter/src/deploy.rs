use std::path::PathBuf;

use cosmwasm_schema::cw_serde;
use cw20::MinterResponse;
use cw721_base::interface::Cw721;
use cw_orch::{contract::Deploy, prelude::*};
use cw_plus_interface::cw20_base::{self, Cw20Base};

use crate::{msg::InstantiateMsg, MinterContract, MinterQueryMsgFns};

pub struct CwOrchWorkshop<Chain> {
    pub nft: Cw721<Chain>,
    pub token: Cw20Base<Chain>,
    pub minter: MinterContract<Chain>,
}

#[cw_serde]
pub struct DeployData {
    pub token_name: String,
    pub token_symbol: String,
    pub token_decimals: u8,
    pub token_minter: String,

    pub native_denom: String,
    pub native_price: u128,
    pub token_price: u128,
}

// Quest #4.1
// Implement the `Deploy` trait for the `CwOrchWorkshop` struct.
// This struct will hold all the contracts needed to deploy the NFT project and the minter.
// I explain the specifics of each function in a comment. I'm letting you implement everything yourself (up to some details)
// When you thing everything is ready, go to Quest 4.2 to test your code !
impl<Chain: CwEnv> Deploy<Chain> for CwOrchWorkshop<Chain> {
    type Error = CwOrchError;

    type DeployData = DeployData;

    // This function deploys the whole bundle on the `chain` environment. It uploads and instantiates all the necessary contracts and do all the necessary steps
    // It uses the `data` argument to pass extra parameters or information needed for deployment.
    // Here we pass all the useful parameters needed for uploading and instantiating the contracts
    // Don't hesitate to play with the values obviously!
    // The following tips work a little like solutions, don't read them if you're strong enough !
    // Tips :
    // - This function usually starts by uploading all the codes
    // - You can then instantiate the cw20 token
    // - Then the minter contract
    // - Don't forget to set the nft address inside the struct because this is not automatic!
    fn deploy_on(chain: Chain, data: Self::DeployData) -> Result<Self, Self::Error> {
        todo!()
    }

    // This function uploads all the necessary contracts.
    // 1. Upload all the contracts,
    // 2. Return the constructed struct
    fn store_on(chain: Chain) -> Result<Self, Self::Error> {
        todo!()
    }

    fn deployed_state_file_path() -> Option<String> {
        let crate_path = env!("CARGO_MANIFEST_DIR");

        Some(
            PathBuf::from(crate_path)
                // State file of your deployment
                .join("state.json")
                .display()
                .to_string(),
        )
    }

    // This function returns all the contracts used inside of this bundle
    // To be able to return any contract type, wrap them in a box :
    // `Box::new(&mut self.contract)`
    fn get_contracts_mut(
        &mut self,
    ) -> Vec<Box<&mut dyn cw_orch::prelude::ContractInstance<Chain>>> {
        todo!()
    }

    // This function loads all the contract from the current chain
    // This doesn't have to verify that the contracts are available on chain and in the local state but it can.
    //
    // 1. Upload all the contracts,
    // 2. Return the constructed struct
    fn load_from(chain: Chain) -> Result<Self, Self::Error> {
        todo!();

        // This is very important to be able to export your state (code-ids, addresses) correctly
        // More on this here: https://orchestrator.abstract.money/setup/workspace/collaboration.html#state-file
        // Un-comment this line when you have your implementation ready above
        // bundle.set_contracts_state(None);

        // Return the bundle
        // Ok(bundle)
    }
}
