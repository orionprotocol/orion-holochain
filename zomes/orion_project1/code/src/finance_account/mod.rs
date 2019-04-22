use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};

struct FinanceAccount {
    currency: String,
    amount: i64,
    broker_id: i64,
    inserted_at: i64
}