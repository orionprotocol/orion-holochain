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
    dna::entry_types::Sharing

};
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
struct Order {
    exchange_addr: HashString,
    broker_addr: HashString,
    base_asset_code: String,
    quoted_asset_code: String,
    direction: Direction,
    quoted_price_per_unit: f64,
    amount: f64,
    inserted_at: u64
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
enum Direction {
    Buy,
    Sell
}

pub fn definition() -> ValidatingEntryType {
    entry!(
      name: "order",
      description: "",
      sharing: Sharing::Public,
      validation_package: || {
        hdk::ValidationPackageDefinition::Entry
      },
      validation: |validation_data: hdk::EntryValidationData<Order>| {
          Ok(())
      },

      links: [
          to!(
              "transaction",
              tag: "transactions",
              validation_package: || {
                  hdk::ValidationPackageDefinition::Entry
              },
              validation: |_validation_data: hdk::LinkValidationData| {
                  Ok(())
              }
          ),
          from!(
            "broker",
            tag: "broker",
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

impl Order {

  // todo
  fn new() -> Self {
    unimplemented!()
    // Order{
    //   exchange_addr: HashString,
    //   broker_addr: HashString,
    //   base_asset_code: String,
    //   quoted_asset_code: String,
    //   direction: Direction,
    //   quoted_price_per_unit: f64,
    //   quantity: f64,

    //   /*todo - now()*/
    //   inserted_at: -1
    // }
  }

    fn calculate_total_price(self) -> f64 {
        self.amount * self.quoted_price_per_unit
    }
}



pub fn handle_get(addr: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&addr)
}

// todo
pub fn handle_approve(addr: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&addr);
    unimplemented!()
}

pub fn handle_create() {
    unimplemented!()
}

//status of the most recent trade
fn get_status(addr: Address) {
  unimplemented!()

}