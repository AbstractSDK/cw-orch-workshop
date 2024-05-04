#![warn(missing_docs)]
//! # Counter contract

use cosmwasm_schema::{cw_serde, QueryResponses};

// QUEST #1.3
// Simplify the interactions with your interface
// cw-orch provides a simplification of the interface with smart-contracts
// These simplifications turn your endpoints into function that you can call on the smart-contract
// Learn more in the official documentation : https://orchestrator.abstract.money/contracts/entry-points.html
// Quest : Replace the `**<<>>**` placeholder with a derive macro to be able to call execute messages
// Verification : Run `cargo run --example interact` successfully to verify quest 1.1, 1.2 and 1.3 are complete
// You have finished QUEST 1 !
// Go to QUEST #2.1 once this task is complete

#[cw_serde]
/// Instantiate method for counter
pub struct InstantiateMsg {
    /// Initial count
    pub count: i32,
}

// ANCHOR: exec_msg
#[cw_serde]
#[derive(**<<>>**)] // Function generation
/// Execute methods for counter
pub enum ExecuteMsg {
    /// Increment count by one
    Increment {},
    /// Reset count
    Reset {
        /// Count value after reset
        count: i32,
    },
}
// ANCHOR_END: exec_msg

// ANCHOR: query_msg
#[cw_serde]
#[derive(cw_orch::QueryFns)] // Function generation
#[derive(QueryResponses)]
/// Query methods for counter
pub enum QueryMsg {
    /// GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
}

// Custom response for the query
#[cw_serde]
/// Response from get_count query
pub struct GetCountResponse {
    /// Current count in the state
    pub count: i32,
}
// ANCHOR_END: query_msg

#[cw_serde]
/// Migrate message for count contract
pub struct MigrateMsg {
    /// Your favorite type of tea
    pub t: String,
}
