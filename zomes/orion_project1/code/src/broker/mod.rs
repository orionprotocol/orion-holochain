use hdk::entry_definition::ValidatingEntryType;
use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    error::HolochainError,
    json::{JsonString,RawString},
    hash::HashString,
    dna::entry_types::Sharing
};


#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
struct Broker {
    name: String
}

pub fn definition() -> ValidatingEntryType {
    unimplemented!()

}

pub fn handle_create(name: &str) -> ZomeApiResult<Address> {
    let brk = Broker{name: name.into()};
    let entry = Entry::App("broker".into(), brk.into());
    let new_addr = hdk::commit_entry(&entry)?;
    Ok(new_addr)
}

pub fn get(addr: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&addr)
}

fn trade(balance_addr: HashString, order_addr: HashString, ) {
  unimplemented!()
}
