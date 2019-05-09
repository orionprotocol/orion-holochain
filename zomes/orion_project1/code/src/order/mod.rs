use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    error::ZomeApiError
};
use hdk::holochain_core_types::{
    cas::content::Address,
    hash::HashString,
    entry::Entry,
    error::HolochainError,
    json::{JsonString,RawString},
    dna::entry_types::Sharing
};
use std::time::{SystemTime, UNIX_EPOCH};


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
pub enum Direction {
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
  fn new(base_asset_code: String, quoted_asset_code: String, direction: Direction, quoted_price_per_unit: f64, amount: f64) -> Self {
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    Order{
      exchange_addr: HashString::default(),
      broker_addr: HashString::default(),
      base_asset_code: base_asset_code,
      quoted_asset_code: quoted_asset_code,
      direction: direction,
      quoted_price_per_unit: quoted_price_per_unit,
      amount: amount,
      inserted_at: ts
    }
  }

    fn calculate_total_price(self) -> f64 {
        self.amount * self.quoted_price_per_unit
    }
}



pub fn handle_get(addr: HashString) -> Result<Option<Entry>, ZomeApiError> {
    hdk::get_entry(&addr)
}

// todo
pub fn handle_approve(addr: HashString) -> Result<(), ZomeApiError> {
    hdk::get_entry(&addr);
    unimplemented!()
}


// todo - replace String with &str
pub fn handle_create(base_asset_code: String, quoted_asset_code: String, direction: Direction, quoted_price_per_unit: f64, amount: f64) -> Result<HashString, ZomeApiError> {
    let ord1 = Order::new(base_asset_code, quoted_asset_code, direction, quoted_price_per_unit, amount);

    let ord1_ent = Entry::App("order".into(), ord1.into());
    Ok(hdk::commit_entry(&ord1_ent)?)
}

//status of the most recent trade
fn get_status(addr: HashString) {
  unimplemented!()

}