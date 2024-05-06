use cosmwasm_std::{Deps, StdResult};

use crate::{msg::StateResponse, state::STATE};

pub fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(StateResponse {
        native_denom: state.native_denom,
        native_price: state.native_price,
        cw20_address: state.cw20_address.to_string(),
        cw20_price: state.cw20_price,
        nft_address: state.nft_address.unwrap().to_string(),
    })
}
