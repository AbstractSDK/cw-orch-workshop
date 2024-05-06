//! # Counter contract

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use cw20::Cw20ReceiveMsg;

#[cw_serde]
/// Instantiate method for counter
pub struct InstantiateMsg {
    pub native_denom: String,
    pub native_price: Uint128,
    pub cw20_address: String,
    pub cw20_price: Uint128,
    pub nft_code_id: u64,
}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)] // Function generation
/// Execute methods for counter
pub enum ExecuteMsg {
    /// Mint funds using cw20 token
    Receive(Cw20ReceiveMsg),
    /// Mint funds using native token
    #[payable]
    Mint {},
    /// Update the price of the minted tokens
    UpdatePrice {
        native_denom: Option<String>,
        native_price: Option<Uint128>,
        cw20_address: Option<String>,
        cw20_price: Option<Uint128>,
    },
}

#[cw_serde]
pub enum NftMinterReceiveMsg {
    Mint {},
}

#[cw_serde]
#[derive(cw_orch::QueryFns)] // Function generation
#[derive(QueryResponses)]
/// Query methods for counter
pub enum QueryMsg {
    /// GetCount returns the current count as a json-encoded number
    #[returns(StateResponse)]
    State {},
}

#[cw_serde]
/// Response from nft_price query
pub struct StateResponse {
    pub native_denom: String,
    pub native_price: Uint128,
    pub cw20_address: String,
    pub cw20_price: Uint128,
    pub nft_address: String,
}

#[cw_serde]
/// Empty Migrate message
pub struct MigrateMsg {}
