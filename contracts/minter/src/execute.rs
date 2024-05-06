use cosmwasm_std::{wasm_execute, Addr, Deps, Empty, Env, Response};

use crate::{error::*, state::*};

pub fn _mint(deps: Deps, env: Env, receiver: Addr) -> Result<Response, ContractError> {
    // We only allow one token per addr per block
    let token_id = format!("block:{}addr:{}", env.block.height, receiver);
    let state = STATE.load(deps.storage)?;
    Ok(Response::new().add_message(wasm_execute(
        state.nft_address.unwrap(),
        &cw721_base::ExecuteMsg::<Option<Empty>, Empty>::Mint {
            token_id,
            owner: receiver.to_string(),
            token_uri: None,
            extension: None,
        },
        vec![],
    )?))
}
