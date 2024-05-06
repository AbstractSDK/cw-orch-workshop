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

impl<Chain: CwEnv> Deploy<Chain> for CwOrchWorkshop<Chain> {
    type Error = CwOrchError;

    type DeployData = DeployData;

    fn deploy_on(chain: Chain, data: Self::DeployData) -> Result<Self, Self::Error> {
        let bundle = Self::store_on(chain.clone())?;
        bundle.token.instantiate(
            &cw20_base::InstantiateMsg {
                name: data.token_name,
                symbol: data.token_symbol,
                decimals: data.token_decimals,
                initial_balances: vec![],
                mint: Some(MinterResponse {
                    minter: data.token_minter,
                    cap: None,
                }),
                marketing: None,
            },
            None,
            None,
        )?;

        bundle.minter.instantiate(
            &InstantiateMsg {
                native_denom: data.native_denom.to_string(),
                native_price: data.native_price.into(),
                cw20_address: bundle.token.address()?.to_string(),
                cw20_price: data.token_price.into(),
                nft_code_id: bundle.nft.code_id()?,
            },
            None,
            None,
        )?;

        let state = bundle.minter.state()?;
        bundle.nft.set_address(&Addr::unchecked(state.nft_address));

        Ok(bundle)
    }

    fn store_on(chain: Chain) -> Result<Self, Self::Error> {
        let Self { nft, token, minter } = Self::load_from(chain)?;

        nft.upload()?;
        token.upload()?;
        minter.upload()?;

        Ok(Self { nft, token, minter })
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

    fn get_contracts_mut(
        &mut self,
    ) -> Vec<Box<&mut dyn cw_orch::prelude::ContractInstance<Chain>>> {
        vec![
            Box::new(&mut self.nft),
            Box::new(&mut self.token),
            Box::new(&mut self.minter),
        ]
    }

    fn load_from(chain: Chain) -> Result<Self, Self::Error> {
        let nft = Cw721::new("nft", chain.clone());
        let token = Cw20Base::new("token", chain.clone());
        let minter = MinterContract::new(chain.clone());

        let mut bundle = Self { nft, token, minter };
        // This is very important to be able to export your code correctly
        bundle.set_contracts_state(None);

        Ok(bundle)
    }
}
