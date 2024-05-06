use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

#[cosmwasm_schema::cw_serde]
pub struct State {
    pub native_denom: String,
    pub native_price: Uint128,
    pub cw20_address: Addr,
    pub cw20_price: Uint128,
    pub nft_address: Option<Addr>,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");
