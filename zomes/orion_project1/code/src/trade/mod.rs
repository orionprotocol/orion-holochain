use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::ZomeApiError
};
use holochain_core_types_derive::DefaultJson;
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    error::HolochainError,
    json::JsonString,
    hash::HashString,
    validation::EntryValidationData,
    dna::entry_types::Sharing
};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Trade {
    order_addr: HashString,
    price: f64,
    asset_code: String,
    status: Status,
    inserted_at: u64
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
    fn new(order_addr: HashString, price: f64, asset_code: &str) -> Option<Self> {
        // todo - check if 'order' exists
        // todo - check price less or equal to the price of 'order'
        // todo - check asset_code


        match hdk::get_entry(&order_addr) {
            Ok(source_order_raw) => {
                if let Some(Entry::App(_, json_str)) = source_order_raw {
                    // todo: for simplicity
                    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

                    Some(Trade {
                        order_addr: order_addr,
                        price: price,
                        asset_code: asset_code.into(),
                        status: Status::New, //todo
                        inserted_at: ts
                    })
                } else {
                    None
                }
            },
            Err(err) => {
                unimplemented!()
            }
        }
    }
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "trade",
        description: "trade or transaction",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        // todo
        validation: |data: hdk::EntryValidationData<Trade>| {
            Ok(())
        },

        // todo
        links: [
            from!(
                "order",
                tag: "order",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}

pub fn handle_create() -> Result<HashString, ZomeApiError> {
    unimplemented!()
}