use hdk::entry_definition::ValidatingEntryType;
use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry,
};

struct Broker {
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
    //todo verify signature
    //
    //
    let brk = Broker {name: name};
    let entry = Entry::App("broker".into(), brk.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn get(addr: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}

fn trade(balance_addr: HashString, order_addr: HashString, ) {
  unimplemented!()
}


// todo
pub fn sign_message(key_id: String, message: String) -> ZomeApiResult<Signature> {
    if key_id == "" {
        hdk::sign(message).map(Signature::from)
    } else {
        hdk::keystore_sign(key_id, message).map(Signature::from)
    }
}

pub fn verify_message(message: String, provenance: Provenance) -> ZomeApiResult<bool> {
    hdk::verify_signature(provenance, message)
}

