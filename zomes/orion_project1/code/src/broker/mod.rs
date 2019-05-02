use hdk::entry_definition::ValidatingEntryType;
use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    error::HolochainError,
    json::{JsonString,RawString},
    hash::HashString,
};


struct Broker {
    name: String
}

pub fn definition() -> ValidatingEntryType {
    entry!(
      name: "broker",
      description: "a middle man between a user and an exchange",
      sharing: Sharing::Public,
      validation_package: || hdk::ValidationPackageDefinition::Entry,
      validation: |validation_data: hdk::EntryValidationData| {
          Ok(())
      },

      links: [
          //todo: not "from!" ?
          to!(
              "%agent_id",
              tag: "owner",
              validation_package: || hdk::ValidationPackageDefinition::Entry,
              validation:  | _validation_data: hdk::LinkValidationData| {
                  Ok(())
              }
          ),

          to!(
              "balance",
              tag: "balances",
              validation_package: || hdk::ValidationPackageDefinition::Entry,
              validation: |_validation_data: hdk::LinkValidationData| {
                  Ok(())
              }
          ),

          to!(
              "order",
              tag: "open_orders",
              validation_package: || hdk::ValidationPackageDefinition::Entry,
              validation: |_validation_data: hdk::LinkValidationData| {
                  Ok(())
              }
          )

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

pub fn handle_create(name: &str) -> ZomeApiResult<Address> {
    let brk = Broker{name: name.into()};
    let entry = Entry::App("broker".into(), brk.into());
    let new_addr = hdk::commit_entry(&entry)?;
    Ok(new_addr)
}

pub fn get(addr: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&addr)
}

fn trade(balance_addr: HashString, order_addr: HashString, ) {
  unimplemented!()
}
