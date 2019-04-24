use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};

struct Balance {
    id: HashString, //needed?
    asset_code: String,
    amount: i64,
    broker_id: HashString,
    inserted_at: i64,
    updated_at: i64
}

pub fn definition() -> ValidatingEntryType {
    entry!(
      name: "balance",
      description: "",
      sharing: Sharing::Public,
      native_type: Balance,
      validation_package: || hdk::ValidationPackageDefinition::Entry,
      validation: |validation_data: hdk::EntryValidationData| {
          Ok(())
      },

      links: [
          from!(
              "broker",
              tag: "broker",
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
pub fn hande_create(blc: Balance, broker_addr: HashString) -> ZomeApiResult<Address> {
    let blc_entry = Entry::App("balance".into(), blc.into());
    let blc_addr = hdk::commit_entry(&blc_entry)?;
    hdk::link_entries(&broker_addr, &blc_addr, "balances")?;
    Ok(blc_addr)
}