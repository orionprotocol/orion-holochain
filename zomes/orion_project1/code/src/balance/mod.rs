use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};

struct Balance {
    id: HashString,
    asset_code: String,
    amount: i64,
    broker_id: i64,
    inserted_at: i64,
    updated_at: i64
}