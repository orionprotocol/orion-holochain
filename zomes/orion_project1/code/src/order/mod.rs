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

struct Order {
    exchange_addr: HashString,
    broker_addr: HashString,
    base_asset_code: String,
    quoted_asset_code: String,
    direction: Direction,

    //todo: these may be replaced with 'decimal' if needed
    quoted_price_per_unit: f64,
    amount: f64,

    inserted_at: i64
}

enum Direction {
    Buy,
    Sell
}

pub fn definition() -> ValidatingEntryType {
    entry!(
      name: "order",
      description: "",
      sharing: Sharing::Public,
      native_type: Order,
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

impl Order {
  fn new() -> Self {
    Order{
      exchange_id: HashString,
      broker_id: HashString,
      base_asset_code: String,
      quoted_asset_code: String,
      direction: Direction,
      quoted_price_per_unit: f64,
      quantity: f64,

      inserted_at: /*todo - now()*/
    }
  }
}



fn handle_get(addr: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&addr)
}


fn calculate_total_price(ord: Order) -> f64 {
    ord.quantity * ord.quoted_price_per_unit
}

fn handle_create() {

  unimplemented!()
}

fn handle_accept() {
  unimplemented!()
}

//status of the most recent transaction
fn get_status(addr: Address) {
  unimplemented!()

}