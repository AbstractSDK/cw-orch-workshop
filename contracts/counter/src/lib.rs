pub mod contract;
mod error;
pub(crate) mod execute;
pub mod msg;
pub(crate) mod query;
pub mod state;

pub use crate::error::ContractError;
pub use crate::msg::QueryMsgFns as CounterQueryMsgFns;

#[cfg(not(target_arch = "wasm32"))]
mod interface;

#[cfg(not(target_arch = "wasm32"))]
pub use crate::interface::CounterContract;
