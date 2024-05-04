use counter_contract::{
    msg::CounterExecuteMsgFns, msg::InstantiateMsg, CounterContract, CounterQueryMsgFns,
};
use cw_orch::{anyhow, prelude::*};

pub fn main() -> anyhow::Result<()> {
    let chain = MockBech32::new("mock");

    let counter = CounterContract::new(chain);

    counter.upload()?;
    counter.instantiate(&InstantiateMsg { count: 0 }, None, None)?;

    counter.increment()?;

    let count = counter.get_count()?;
    assert_eq!(count.count, 1);
    Ok(())
}
