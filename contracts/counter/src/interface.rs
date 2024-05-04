use cw_orch::environment::ChainInfoOwned;
// ANCHOR: custom_interface
use cw_orch::{interface, prelude::*};

use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

pub const CONTRACT_ID: &str = "counter_contract";

// QUEST #1.1
// Definition of the cw-orch interface
// A cw-orch contract interface is defined using cw-orch::interface macro.
// You *will* specify all the interface messages used inside that contract
// You *CAN* specify an optional contract identifier
// For help, visit the official documentation : https://orchestrator.abstract.money/contracts/interfaces.html
// Go to QUEST #1.2 once this task is complete
pub struct CounterContract;

// QUEST #1.2
// Link the smart-contract to the cw-orch interface
// After definition, a cw-orch interface needs to be linked to its actual implementation and WASM
// You do that inside the `Uploadable` trait implementation.
// Quest : Replace the `**<<>>**` placeholders with actual values to complete the interface
// Verification : Run `cargo run --example upload` successfully to verify quest 1.1 and 1.2 are complete
// Go to QUEST #1.3 once this task is complete
impl<Chain> Uploadable for CounterContract<Chain> {
    /// Return the path to the wasm file corresponding to the contract
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("**<<>>**")
            .unwrap()
    }
    /// Returns a CosmWasm contract wrapper
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                crate::contract::execute,
                **<<>>**,
                crate::contract::query,
            )
            .with_migrate(crate::contract::migrate),
        )
    }
}
