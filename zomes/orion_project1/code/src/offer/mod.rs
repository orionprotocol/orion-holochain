use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult
};

use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    error::HolochainError,
    json::{JsonString,RawString},
    hash::HashString,
};

struct Offer {
    id: HashString,
    exchange_id: i64,
    broker_id: HashString,
    base_asset_code: String,
    quoted_asset_code: String,
    direction: Direction,

    //todo: these may be replaced with 'decimal' if needed
    price: f64,
    quantity: f64,

    inserted_at: i64
}

enum Direction {
  Buy,
  Sell
}