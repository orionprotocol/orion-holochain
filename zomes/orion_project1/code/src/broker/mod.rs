use hdk::entry_definition::ValidatingEntryType;
use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError
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
struct Broker {
    name: String
}

pub fn definition() -> ValidatingEntryType {
    entry!(
      name: "broker",
      description: "a middle man between a user and an exchange",
      sharing: Sharing::Public,
      validation_package: || {
        hdk::ValidationPackageDefinition::Entry
      },
      validation: |validation_data: hdk::EntryValidationData<Broker>| {
          Ok(())
      },

      links: [
          //todo: not "from!" ?
          to!(
              "%agent_id",
              tag: "owner",
              validation_package: || {
                hdk::ValidationPackageDefinition::Entry
              },
              validation:  | _validation_data: hdk::LinkValidationData| {
                  Ok(())
              }
          ),

          to!(
              "balance",
              tag: "balances",
              validation_package: || {
                hdk::ValidationPackageDefinition::Entry
              },
              validation: |_validation_data: hdk::LinkValidationData| {
                  Ok(())
              }
          ),

          to!(
              "order",
              tag: "open_orders",
              validation_package: || {
                hdk::ValidationPackageDefinition::Entry
              },
              validation: |_validation_data: hdk::LinkValidationData| {
                  Ok(())
              }
          ),

          to!(
              "order",
              tag: "closed_orders",
              validation_package: || hdk::ValidationPackageDefinition::Entry,
              validation: |_validation_data: hdk::LinkValidationData| {
                  Ok(())
              }
          )
      ]
    )
}

// pub fn handle_create(name: &str) -> ZomeApiResult<Address> {
pub fn handle_create(name: String) -> Result<HashString, ZomeApiError> {
    // let brk = Broker{name: name.into()};
    // let entry = Entry::App("broker".into(), brk.into());
    // let new_addr = hdk::commit_entry(&entry)?;
    // Ok(new_addr);

    unimplemented!()

}

pub fn get(addr: HashString) -> Result<Option<Entry>, ZomeApiError> {
    hdk::get_entry(&addr)
}

fn trade(balance_addr: HashString, order_addr: HashString) {
    unimplemented!()
}
