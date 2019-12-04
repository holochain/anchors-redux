#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
};

use hdk::holochain_json_api::{
    json::JsonString,
    error::JsonError
};

use hdk::holochain_persistence_api::{
    cas::content::Address
};
//use serde_json::json;
use hdk_proc_macros::zome;
//use std::convert::TryInto;

use hdk::prelude::LinkMatch;

// see https://developer.holochain.org/api/0.0.38-alpha14/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MotorCycle" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct MotorCycle {
    name: String,
}

#[zome]
mod my_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn anchor_def() -> ValidatingEntryType {
        holochain_anchors::anchor_definition()
    }
    
    #[entry_def]
    fn root_anchor_def() -> ValidatingEntryType {
        holochain_anchors::root_anchor_definition()
    }

    #[entry_def]
     fn motorcycle_entry_def() -> ValidatingEntryType {
        entry!(
            name: "motorcycle",
            description: "A motorcycle",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<MotorCycle>| {
                Ok(())
            },
            links: [
                from!(
                    holochain_anchors::ANCHOR_TYPE,
                    link_type: "motorcycle_link_to",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },

                    validation: |_validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    }

    #[zome_fn("hc_public")]
    fn add_motorcycle(name: String) -> ZomeApiResult<Address> {
        let motorcycle_entry = Entry::App(
            "motorcycle".into(),
             MotorCycle {
                name
            }.into()
        );
        let address = hdk::commit_entry(&motorcycle_entry)?;
        let anchor_address = holochain_anchors::create_anchor("model".into(), "soft-tail".into())?;
        hdk::link_entries(&anchor_address, &address, "motorcycle_link_to", "")?;

        Ok(address)
    }

    #[zome_fn("hc_public")]
    fn get_motorcycle(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }

    #[zome_fn("hc_public")]
    fn get_motorcycles(anchor_type: String, anchor_text: String) -> ZomeApiResult<Vec<MotorCycle>> {
        let anchor_address = holochain_anchors::create_anchor(anchor_type, anchor_text)?;
        hdk::utils::get_links_and_load_type(&anchor_address, LinkMatch::Exactly("motorcycle_link_to"), LinkMatch::Any)
    }

}
