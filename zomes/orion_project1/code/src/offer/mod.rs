use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};


struct Offer {
    id: i64,
    exchange_id: i64,
    currency: String, //the code
    quantity: i64,
    price: i64, //todo - consider crate currency 
    inserted_at: i64
}
