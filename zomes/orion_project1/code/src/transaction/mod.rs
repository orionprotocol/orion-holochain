use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};

//todo: or Trade
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Transaction {
    id: i64,
    offer_id: i64,
    price: i64,
    currency_code: String,
    status: Status,
    inserted_at: i64
}

enum Status {
    New,
    Filled,
    Canceled,
    PartiallyFilled,
    PartiallyCancelled
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
