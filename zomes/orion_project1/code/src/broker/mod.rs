use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry,
};

pub fn create(name: String, signature: String) -> ZomeApiResult<Address> {
    //todo verify signature
    //
    //

    let entry = Entry::App(name);
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}
