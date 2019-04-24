#![allow(unused_variables)]
#![feature(try_from)]
#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_core_types::{
        hash::HashString,
        error::HolochainError,
        dna::entry_types::Sharing,
        json::JsonString,
        cas::content::Address,
        entry::Entry,
    }
};

pub mod broker;
pub mod transaction;

define_zome! {
    entries: [
        broker::definition(),
        balance::definition(),
        order::definition(),
        transaction::definition()
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
            handler: ofder::handle_create
        }

        approve_order: {
            inputs: |addr: HashString|,
            outputs: |result: std::result::Result<(), ZomeApiError>|,
            handler: order::handle_approve
        }

        //transaction is also called 'trade'
        create_transaction: {
            inputs: |/*todo*/|,
            outputs: |result: std::result::Result<Address, ZomeApiError>|,
            handler: transaction::handle_create
        }
    ]

    traits: {
        hc_public [register_broker]
    }
}
