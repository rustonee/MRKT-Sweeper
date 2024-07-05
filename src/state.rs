use cw_storage_plus::Item;
use crate::msg::{ExpectedPrice, Nft, State};

pub const STATE: Item<State> = Item::new("STATE");
pub const EXPECTED_PRICE: Item<ExpectedPrice> = Item::new("EXPECTED_PRICE");
pub const NFT: Item<Nft> = Item::new("NFT");
