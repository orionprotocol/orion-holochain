use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry,
};

struct Broker {
    id: HashString,
    name: String,
    signed_pubkey: String
}

pub fn definition() -> ValidatingEntryType {
    entry!(
      name: "broker",
      description: "a middle man between a user and an exchange",
      sharing: Sharing::Public,
      native_type: Broker,
      validation_package: || hdk::ValidationPackageDefinition::Entry,
      validation: |validation_data: hdk::EntryValidationData| {
          Ok(())
      },

      links: [
          to!(
              "balance",
              tag: "balances",
              validation_package: || hdk::ValidationPackageDefinition::Entry,
              validation: |_validation_data: hdk::LinkValidationData| {
                  Ok(())
              }
          )
      ]
    )
}

pub fn handle_create(name: &str, signature: &str) -> ZomeApiResult<Address> {
    //todo verify signature
    //
    //

    let entry = Entry::App(name);
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn get(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}

fn trade() {
  unimplemented!()
}
