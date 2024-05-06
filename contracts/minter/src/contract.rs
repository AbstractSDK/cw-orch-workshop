use cosmwasm_std::{
    ensure_eq, entry_point, to_json_binary, wasm_instantiate, Binary, Deps, DepsMut, Env,
    MessageInfo, Reply, Response, StdError, StdResult, SubMsg,
};
use cw2::set_contract_version;
use cw_utils::one_coin;

use crate::{error::*, execute::_mint, msg::*, query::query_state, state::*};

// version info for migration info
pub const CONTRACT_NAME: &str = "crates.io:counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// ANCHOR: interface_entry
// ANCHOR: entry_point_line
#[cfg_attr(feature = "export", entry_point)]
// ANCHOR_END: entry_point_line
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        native_denom: msg.native_denom,
        native_price: msg.native_price,
        cw20_address: deps.api.addr_validate(&msg.cw20_address)?,
        cw20_price: msg.cw20_price,
        nft_address: None,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    let cw721_instantiate_msg = wasm_instantiate(
        msg.nft_code_id,
        &cw721_base::InstantiateMsg {
            name: "cw-orch NFT".to_string(),
            symbol: "CWORCH".to_string(),
            minter: Some(env.contract.address.to_string()),
            withdraw_address: None,
        },
        vec![],
        "Nft Contract".to_string(),
    )?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_submessage(SubMsg::reply_on_success(cw721_instantiate_msg, CW721_REPLY)))
}

#[cfg_attr(feature = "export", entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Receive(msg) => {
            let state = STATE.load(deps.storage)?;

            ensure_eq!(
                state.cw20_address,
                info.sender,
                StdError::generic_err("Not authorized cw20 address")
            );

            ensure_eq!(
                state.cw20_price,
                msg.amount,
                StdError::generic_err("Not enough paiement")
            );

            _mint(deps.as_ref(), env, deps.api.addr_validate(&msg.sender)?)
        }
        ExecuteMsg::Mint {} => {
            let state = STATE.load(deps.storage)?;

            let funds = one_coin(&info)?;

            ensure_eq!(
                state.native_denom,
                funds.denom,
                StdError::generic_err("Not authorized native token")
            );

            ensure_eq!(
                state.native_price,
                funds.amount,
                StdError::generic_err("Not enough paiement")
            );

            _mint(deps.as_ref(), env, info.sender)
        }
        ExecuteMsg::UpdatePrice {
            native_denom,
            native_price,
            cw20_address,
            cw20_price,
        } => {
            let mut state = STATE.load(deps.storage)?;
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            if let Some(native_denom) = native_denom {
                state.native_denom = native_denom
            }
            if let Some(native_price) = native_price {
                state.native_price = native_price
            }
            if let Some(cw20_address) = cw20_address {
                state.cw20_address = deps.api.addr_validate(&cw20_address)?;
            }
            if let Some(cw20_price) = cw20_price {
                state.cw20_price = cw20_price
            }
            STATE.save(deps.storage, &state)?;

            Ok(Response::new())
        }
    }
}

#[cfg_attr(feature = "export", entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::State {} => to_json_binary(&query_state(deps)?),
    }
}

pub const CW721_REPLY: u64 = 24;

pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        CW721_REPLY => {
            let result = reply.result.into_result().map_err(StdError::generic_err)?;
            let instantiate_data = cw_utils::parse_instantiate_response_data(&result.data.unwrap())
                .map_err(|e| StdError::generic_err(e.to_string()))?;
            let mut state = STATE.load(deps.storage)?;
            state.nft_address = Some(deps.api.addr_validate(&instantiate_data.contract_address)?);
            STATE.save(deps.storage, &state)?;

            Ok(Response::new())
        }
        _ => Err(ContractError::Std(StdError::generic_err(
            "Reply not supported",
        ))),
    }
}

#[cfg_attr(feature = "export", entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default().add_attribute("action", "migrate"))
}
// ANCHOR_END: interface_entry
