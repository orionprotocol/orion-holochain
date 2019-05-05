#![allow(unused_variables)]
#![feature(try_from)]
#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::holochain_core_types::{
    cas::content::Address,
    error::HolochainError,
    json::JsonString,
    hash::HashString
};
use hdk::{
    error::ZomeApiError
};

pub mod broker;
pub mod trade;
pub mod order;
pub mod balance;


define_zome! {
    entries: [
        broker::definition(),
        balance::definition(),
        order::definition(),
        trade::definition()
    ]

    genesis: || {
        Ok(())
    }

    functions: [
        create_broker: {
            inputs: |name: String|,
            outputs: |result: std::result::Result<HashString, ZomeApiError>|,
            handler: broker::handle_create
        }

        initialize_order: {
            inputs: |/*todo*/|,
            outputs: |result: Result<HashString, ZomeApiError>|,
            handler: order::handle_create
        }

        approve_order: {
            inputs: |addr: HashString|,
            outputs: |result: Result<(), ZomeApiError>|,
            handler: order::handle_approve
        }

        create_trade: {
            inputs: |/*todo*/|,
            outputs: |result: Result<HashString, ZomeApiError>|,
            handler: trade::handle_create
        }
    ]

    traits: {
        hc_public [
            create_broker,
            initialize_order,
            approve_order,
            create_trade
        ]
    }
}
