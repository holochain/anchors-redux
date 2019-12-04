#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
#[macro_use]
extern crate holochain_json_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
// #[macro_use]
// extern crate serde_json;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
};

use hdk::holochain_persistence_api::{
    cas::content::Address
};

use hdk_proc_macros::zome;

pub mod anchors;
use hdk::prelude::*;


pub static ANCHOR_ENTRY: &str = "anchor2";
pub static ROOT_ANCHOR_ENTRY: &str = "root_anchor2";
pub static ROOT_ANCHOR_LINK_TO: &str = "anchors2";

#[zome]
mod anchors {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
     fn anchor_entry_def() -> ValidatingEntryType {
         anchors::anchor_definition()
    }

    #[entry_def]
     fn root_anchor_entry_def() -> ValidatingEntryType {
         anchors::root_anchor_definition()
    }

    #[zome_fn("hc_public")]
    fn create_anchor(anchor_type: String, anchor_text: String) -> ZomeApiResult<Address> {
        anchors::handlers::handle_create_anchor(anchor_type, anchor_text)
    }

    #[zome_fn("hc_public")]
    fn get_anchor(anchor_address: Address) -> ZomeApiResult<Option<Entry>> {
        anchors::handlers::handle_get_anchor(anchor_address)
    }

    #[zome_fn("hc_public")]
    fn get_anchors() -> ZomeApiResult<Vec<Address>> {
        anchors::handlers::handle_get_anchors()
    }
}
