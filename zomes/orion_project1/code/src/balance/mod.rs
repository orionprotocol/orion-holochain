use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};

struct Balance {
    id: HashString,
    asset_code: String,
    amount: i64,
    broker_id: HashString,
    inserted_at: i64,
    updated_at: i64
}

pub fn definitions() -> ValidatingEntryType {
    entry!(
      name: "balance",
      description: "",
      sharing: Sharing::Public,
      native_type: Balance,
      validation_package: || hdk::ValidationPackageDefinition::Entry,
      validation: |validation_data: hdk::EntryValidationData| {
          Ok(())
      },

      // todo
      links: [
          to!(
              "listItem",
              tag: "items",
              validation_package: || hdk::ValidationPackageDefinition::Entry,
              validation: |_validation_data: hdk::LinkValidationData| {
                  Ok(())
              }
          )
      ]
    ),

}
fn handle_update() {
  unimplemented!()
}