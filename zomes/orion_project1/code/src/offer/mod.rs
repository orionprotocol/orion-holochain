use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};


struct Offer {
    id: i64,
    exchange_id: i64,
    broker_id: i64,
    currency_code: String,
    quantity: i64,
    price: i64,
    inserted_at: i64
}
