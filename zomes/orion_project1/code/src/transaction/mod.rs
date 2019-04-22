use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};

//todo: rename back to Trade?
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Transaction {
  //todo
}


pub fn definitions() -> ValidatingEntryType{
    entry!(
        name: "transaction",
        description: "transaction or trade",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |validation_data: hdk::EntryValidationData<KeyAnchor>| {
            match validation_data
            {
                EntryValidationData::Create{entry:_domain_name,validation_data:_} =>
                {
                    Ok(())
                },
                EntryValidationData::Modify{new_entry:_,old_entry:_,old_entry_header:_,validation_data:_} =>
                {
                   Ok(())
                },
                EntryValidationData::Delete{old_entry:_,old_entry_header:_,validation_data:_} =>
                {
                   Ok(())
                }
            }

        },
        links: []
    )
}


enum Status {
    New,
    Filled,
    Canceled,
    PartiallyFilled,
    PartiallyCancelled
}

struct Transaction {
    id: i64,
    status: Status,
    order_id: i64,
    price: i64,
    quantity: i64, //???
    inserted_at: i64
}
