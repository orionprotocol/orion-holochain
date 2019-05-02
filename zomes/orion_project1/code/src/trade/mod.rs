use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};
use holochain_core_types_derive::DefaultJson;
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    hash::HashString,
    validation::EntryValidationData
};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Trade {
    order_addr: HashString,
    price: f64,
    asset_code: String,
    status: Status,
    inserted_at: i64
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
enum Status {
    New,
    Filled,
    Canceled,
    PartiallyFilled,
    PartiallyCancelled
}

impl Trade {
    fn new(order_addr: HashString, price: f64, asset_code: &str) -> Self {
        // todo - check if 'order' exists
        // todo - check price less or equal to the price of 'order'
        // todo - check asset_code


        // todo - extract
        let source_order_raw = hdk::get_entry(&order_addr)?;
        if let Some(Entry::App(_, json_str)) = source_order_raw {
            // todo: for simplicity
            Some(ts) = SystemTime::now().duration_since(UNIX_EPOCH);

            Trade {
                order_addr: order_addr,
                price: price,
                asset_code: asset_code.into(),
                status: Status::New, //todo
                inserted_at: ts
            }
        } else {
            //todo
            unimplemented!()
        }

    }
}


pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "trade",
        description: "trade or transaction",
        sharing: Sharing::Public,
        native_type: Trade,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        // todo
        validation: |data: hdk::EntryValidationData<KeyAnchor>| {
            match data {
                EntryValidationData::Create{entry:_domain_name, data: _} => {
                    Ok(())
                },
                EntryValidationData::Modify{new_entry:_,old_entry:_,old_entry_header:_, data: _} => {
                   Ok(())
                },
                EntryValidationData::Delete{old_entry:_,old_entry_header:_, data: _} => {
                   Ok(())
                }
            }
        },

        links: [
            from!(
                "order",
                tag: "order"
            ),
        ]
    )
}

pub fn handle_create() {
    unimplemented!()
}