#![allow(unused_variables)]
#![feature(try_from)]
#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

pub mod broker;
pub mod trade;
pub mod order;
pub mod balance;
use hdk::holochain_core_types::{
    cas::content::Address,
    error::HolochainError,
    json::JsonString,
    hash::HashString
};
use hdk::{
    error::ZomeApiError
};


define_zome! {
    entries: [
        broker::definition()
    ]

    genesis: || {
        Ok(())
    }

    functions: [
        create_broker: {
            inputs: |name: String|,
            outputs: |result: std::result::Result<Address, ZomeApiError>|,
            handler: broker::handle_create
        }

        initialize_order: {
            inputs: |/*todo*/|,
            outputs: |result: std::result::Result<Address, ZomeApiError>|,
            handler: order::handle_create
        }

        approve_order: {
            inputs: |addr: HashString|,
            outputs: |result: std::result::Result<(), ZomeApiError>|,
            handler: order::handle_approve
        }

        create_trade: {
            inputs: |/*todo*/|,
            outputs: |result: std::result::Result<Address, ZomeApiError>|,
            handler: trade::handle_create
        }
    ]

    traits: {
        hc_public [
            register_broker,
            initialize_order,
            approve_order,
            create_trade
        ]
    }
}
