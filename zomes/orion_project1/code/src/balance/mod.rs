use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};
use hdk::error::ZomeApiResult;
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    error::HolochainError,
    json::{JsonString,RawString},
    hash::HashString,
    dna::entry_types::Sharing
};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Balance {
    asset_code: String,
    amount: i64,
    broker_addr: HashString,
    inserted_at: i64,
    updated_at: i64
}

pub fn definition() -> ValidatingEntryType {
    entry!(
      name: "balance",
      description: "",
      sharing: Sharing::Public,

      // todo
      // ^^^^^^^^^^^ no rules expected this token in macro call
      // native_type: Balance,

      validation_package: || {
        hdk::ValidationPackageDefinition::Entry
      },
      validation: |validation_data: hdk::EntryValidationData<Balance>| {
          Ok(())
      },

      links: [
          from!(
              "broker",
              tag: "the_broker",
              validation_package: || hdk::ValidationPackageDefinition::Entry,
              validation: |_validation_data: hdk::LinkValidationData| {
                  Ok(())
              }
          )
      ]
    )
}

pub fn handle_update() {
    unimplemented!()
}



//create new balance
// todo - needed? merge with 'handle_update'?
pub fn hande_create(blc: Balance, broker_addr: HashString) -> ZomeApiResult<HashString> {
    let blc_entry = Entry::App("balance".into(), blc.into());
    let blc_addr = hdk::commit_entry(&blc_entry)?;
    hdk::link_entries(&broker_addr, &blc_addr, "balances")?;
    Ok(blc_addr)
}