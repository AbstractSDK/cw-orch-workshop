use counter_contract::{msg::InstantiateMsg, CounterContract};
use cw_orch::{anyhow, daemon::networks::OSMOSIS_1, prelude::*};

pub fn main() -> anyhow::Result<()> {
    let chain = MockBech32::new("mock");

    let counter = CounterContract::new(chain);

    counter.upload()?;
    counter.instantiate(&InstantiateMsg { count: 0 }, None, None)?;

    CounterContract::<MockBech32>::wasm(&OSMOSIS_1.into());

    Ok(())
}
