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
    quoted_price_per_unit: f64,
    quantity: f64,

    inserted_at: i64
}

enum Direction {
    Buy,
    Sell
}

pub fn definition() -> ValidatingEntryType {
    entry!(
      name: "offer",
      description: "also called 'order'",
      sharing: Sharing::Public,
      native_type: Offer,
      validation_package: || hdk::ValidationPackageDefinition::Entry,
      validation: |validation_data: hdk::EntryValidationData| {
          Ok(())
      },

      links: [
          to!(
              "transaction",
              tag: "transactions",
              validation_package: || hdk::ValidationPackageDefinition::Entry,
              validation: |_validation_data: hdk::LinkValidationData| {
                  Ok(())
              }
          ),
          from!(
            "broker",
            tag: "broker"

          )
      ]
    )
}

fn calculate_total_price(ofr: Offer) -> f64 {
    ofr.quantity * ofr.quoted_price_per_unit
}

fn handle_create() {
  unimplemented!()
}

fn handle_accept() {
  unimplemented!()
}

//status of the most recent transaction
fn get_status() {
  unimplemented!()

}