use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};

//todo: or Trade
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Transaction {
    id: i64,
    offer_id: HashString,
    price: f64,
    asset_code: String,
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

        // todo
        validation: |data: hdk::EntryValidationData<KeyAnchor>| {
            match data {
                EntryValidationData::Create{entry:_domain_name, data: _} => {
                    Ok(())
                },
                EntryValidationData::Modify{new_entry:_,old_entry:_,old_entry_header:_, data: _} => {
                   Ok(())
                },
                EntryValidationData::Delete{old_entry:_,old_entry_header:_, data: _} => {
                   Ok(())
                }
            }

        },
        links: []
    )
}
