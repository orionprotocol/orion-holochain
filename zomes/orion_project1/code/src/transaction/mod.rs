use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};

use holochain_core_types_derive::DefaultJson;

//todo: or Trade
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Transaction {
    order_addr: HashString,
    price: f64,
    asset_code: String,
    status: Status,
    inserted_at: i64
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
enum Status {
    New,
    Filled,
    Canceled,
    PartiallyFilled,
    PartiallyCancelled
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "transaction",
        description: "also called 'trade'",
        sharing: Sharing::Public,
        native_type: Transaction,
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

        links: [
            from!(
                "order",
                tag: "order"
            ),
        ]
    )
}
