pub mod contract;
mod error;
pub mod msg;
pub mod state;

use cosmwasm_std::{DepsMut, MessageInfo, Response, WasmMsg, to_binary, Deps, Coin, Uint128};
use msg::{ExpectedPrice, Nft, ExecuteMsg, Bid};
use state::STATE;

pub use crate::error::ContractError;

fn validate_fund(
    _deps: Deps,
    info: MessageInfo,
    expected_price: ExpectedPrice
) -> Result<Coin, ContractError> {
    let base_fund = &Coin {
        denom: String::from(expected_price.denom.clone()),
        amount: Uint128::from(0u128),
    };
    let fund = info
        .funds
        .iter()
        .find(|fund| fund.denom == String::from(expected_price.denom.clone()))
        .unwrap_or(base_fund);

    let cost = Uint128::from(u128::from_str_radix(expected_price.amount.as_str(), 10).unwrap());
    if fund.amount < cost {
        return Err(ContractError::InsufficientFund {
            amount: fund.amount,
            required: cost,
        });
    }

    Ok(fund.clone())
}

pub fn buy_now(deps: DepsMut, info: MessageInfo, expected_price: ExpectedPrice, nft: Nft) -> Result<Response, ContractError> {
    validate_fund(deps.as_ref(), info.clone(), expected_price.clone())?;
    let state = STATE.load(deps.storage)?;
    let nft_marketplace_address = &state.nft_marketplace_address.to_string();

    let fund = info.funds.iter().find(|fund| fund.denom == String::from(expected_price.denom.clone())).unwrap();

    let exec_msg = WasmMsg::Execute {
        contract_addr: nft_marketplace_address.clone(),
        msg: to_binary(&ExecuteMsg::BuyNow{expected_price: expected_price.clone(), nft: nft.clone()})?,
        funds: vec![fund.clone()],
    };
   
    Ok(Response::new()
        .add_message(exec_msg)
        .add_attribute("method", "buy_now"))
}

fn validate_batch_bid_fund(
    _deps: Deps,
    info: MessageInfo,
    bids: Vec<Bid>
) -> Result<Coin, ContractError> {
    let base_fund = &Coin {
        denom: String::from("usei"),
        amount: Uint128::from(0u128),
    };
    let fund = info
        .funds
        .iter()
        .find(|fund| fund.denom == String::from("usei"))
        .unwrap_or(base_fund);

    let cost: u128 = bids.iter().map(|item|
        u128::from_str_radix(item.bid_type.buy_now.expected_price.amount.as_str(), 10).unwrap()
    ).sum();

    let cost = Uint128::from(cost);
    if fund.amount < cost {
        return Err(ContractError::InsufficientFund {
            amount: fund.amount,
            required: cost,
        });
    }

    Ok(fund.clone())
}

pub fn batch_bids(deps: DepsMut, info: MessageInfo, bids: Vec<Bid>) -> Result<Response, ContractError> {
    validate_batch_bid_fund(deps.as_ref(), info.clone(), bids.clone())?;
    let state: msg::State = STATE.load(deps.storage)?;
    let nft_marketplace_address = &state.nft_marketplace_address.to_string();

    let fund = info.funds.iter().find(|fund| fund.denom == String::from("usei")).unwrap();

    let exec_msg = WasmMsg::Execute {
        contract_addr: nft_marketplace_address.clone(),
        msg: to_binary(&ExecuteMsg::BatchBids{bids})?,
        funds: vec![fund.clone()],
    };
   
    Ok(Response::new()
        .add_message(exec_msg)
        .add_attribute("method", "batch_bids"))
}

