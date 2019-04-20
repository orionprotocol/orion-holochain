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
        entry!(
            name: "list",
            description: "",
            sharing: Sharing::Public,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |validation_data: hdk::EntryValidationData<List>| {
                Ok(())
            },
            links: [
                to!(
                    "listItem",
                    tag: "items",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |_validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        ),
        entry!(
            name: "listItem",
            description: "",
            sharing: Sharing::Public,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |validation_data: hdk::EntryValidationData<ListItem>| {
                Ok(())
            }
        )
    ]

    genesis: || {
        Ok(())
    }

    functions: [

        /*
        create_list: {
            inputs: |list: List|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_list
        }
        add_item: {
            inputs: |list_item: ListItem, list_addr: HashString|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_add_item
        }
        get_list: {
            inputs: |list_addr: HashString|,
            outputs: |result: ZomeApiResult<GetListResponse>|,
            handler: handle_get_list
        }
        */

        register_broker: {
            inputs:
            outputs:
            handler: register_broker
        }

        update_balance: {
            inputs:
            outputs:
            handler: update_balance
        }

        init_order: {
            inputs:
            outputs:
            handler: 
        }

        accept_order_tx: { 
            inputs:
            outputs:
            handler: 
        }

        trade_tx: { 
            inputs:
            outputs:
            handler: trades            
        }

    ]
    traits: {
        hc_public [create_list, add_item, get_list]
    }
}



struct TradeCurrencyPair(String, String)

struct Broker {
    name: String,
    signed_pubkey: String
}



//todo
struct Balance {
    balance: "Map[String, Long]",
    inserted_at: i64
    signature: String
}


struct Order {
    id: i64,
    exchange_id: i64,
    currency: String, //the code
    quantity: i64,
    price: i64, //todo - consider crate currency 
    inserted_at: i64
}


enum TradeStatus {
    New,
    Filled,
    Canceled,
    PartiallyFilled,
    PartiallyCancelled
}

struct Trade {
    id: i64,
    order_id: i64,
    price: i64,
    quantity: i64,
    inserted_at: i64
}

struct Transaction {
    id: i64,
    trade_id: i64,
    status: TradeStatus,
    inserted_at: i64
}


fn sign(data: &str, key: &str) -> String {
}

fn validate_signature(data: &str, key: &str) -> bool {
}




pub fn registerBroker(name: String) -> Result<i64, Error> {

}

pub fn updateBalance() -> ? {

}

pub fn createOrder() -> ? { }


pub fn acceptOrder() -> ? { }

pub fn trade() -> ? { }
