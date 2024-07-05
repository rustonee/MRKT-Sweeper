#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env,
    MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::{buy_now, batch_bids};
use crate::error::ContractError;
use crate::msg::{ExpectedPriceResponse, NftResponse, StateResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{EXPECTED_PRICE, NFT, STATE};
use crate::msg::{ExpectedPrice, Nft, State};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:buynow";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    let expected_price = ExpectedPrice {
        amount: "0".to_string(),
        denom: "usei".to_string(),
    };

    EXPECTED_PRICE.save(deps.storage, &expected_price)?;

    let nft = Nft {
        address: info.sender.to_string(),
        token_id: "0".to_string(),
    };

    NFT.save(deps.storage, &nft)?;

    STATE.save(
        deps.storage,
        &State {
            nft_marketplace_address: msg.nft_marketplace_address.clone(),
            owner: info.sender.clone().to_string(),
        },
    )?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("nft_marketplace_address", msg.nft_marketplace_address.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::BuyNow {expected_price, nft} => buy_now(deps, info,  expected_price, nft),
        ExecuteMsg::BatchBids {bids} => batch_bids(deps, info, bids),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetExpectedPrice {} => to_binary(&query_expected_price(deps)?),
        QueryMsg::GetNft {} => to_binary(&query_nft(deps)?),
        QueryMsg::GetState {} => to_binary(&query_state(deps)?),
    }
}

fn query_expected_price(deps: Deps) -> StdResult<ExpectedPriceResponse> {
    let state: ExpectedPrice = EXPECTED_PRICE.load(deps.storage)?;
    Ok(ExpectedPriceResponse { amount: state.amount, denom: state.denom })
}

fn query_nft(deps: Deps) -> StdResult<NftResponse> {
    let state: Nft = NFT.load(deps.storage)?;
    Ok(NftResponse { address: state.address, token_id: state.token_id })
}

fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let state: State = STATE.load(deps.storage)?;
    Ok(StateResponse { nft_marketplace_address: state.nft_marketplace_address})
}
