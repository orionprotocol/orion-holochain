#![allow(unused_variables)]
#![feature(try_from)]

#[macro_use]
extern crate hdk;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate holochain_core_types_derive;
use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        hash::HashString,
        error::HolochainError,
        dna::entry_types::Sharing,
        json::JsonString,
        cas::content::Address,
        entry::Entry,
    }
};


//todo - draft
define_zome! {
    entries: [
        main_agent::descriptions(),
        broker::descriptions(),
        balance::descriptions(),
        offer::descriptions(),
        transaction::descriptions(),
    ]

    genesis: || {
        Ok(())
    }

    functions: [
        register_broker: {
            inputs:
            outputs:
            handler: handle_create_broker
        }
    ]
    traits: {
        hc_public [register_broker]
    }
}
