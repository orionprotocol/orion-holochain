use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry,
};

struct Broker {
    name: String,
    signed_pubkey: String
}

pub fn create(name: &str, signature: &str) -> ZomeApiResult<Address> {
    //todo verify signature
    //
    //

    let entry = Entry::App(name);
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn get(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}
