pub mod contract;
mod error;
pub(crate) mod execute;
pub mod msg;
pub(crate) mod query;
pub mod state;

pub use crate::error::ContractError;
pub use crate::msg::{ExecuteMsgFns as MinterExecuteMsgFns, QueryMsgFns as MinterQueryMsgFns};

#[cfg(not(target_arch = "wasm32"))]
pub mod interface;

#[cfg(not(target_arch = "wasm32"))]
pub use crate::interface::MinterContract;

#[cfg(not(target_arch = "wasm32"))] // This one is here to avoid cw-orch landing inside your WASM
pub mod deploy;
