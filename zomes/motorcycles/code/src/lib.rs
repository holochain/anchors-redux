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
    PUBLIC_TOKEN,
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
use serde_json::json;
use hdk_proc_macros::zome;
use std::convert::TryInto;

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
            }
        )
    }

    #[zome_fn("hc_public")]
    fn add_motorcycle(name: String) -> ZomeApiResult<Address> {
        // let motorcycle_entry = Entry::App(
        //     "motorcycle".into(),
        //     MotorCycle {
        //         name
        //     }.into()
        // );
        // let address = hdk::commit_entry(&motorcycle_entry)?;
        let anchor_address = hdk::call(
            hdk::THIS_INSTANCE,
            "anchors",
            Address::from(PUBLIC_TOKEN.to_string()), // this is the Public token for all hc_public zome functions
            "create_anchor",
            json!({"anchor_type": "model", "anchor_text": name}).into()
        )?;
        // Link it to anchor {"anchor_type": "model", "anchor_text": "soft-tail"}
        let anchor_address :Address = anchor_address.try_into()?;
        // hdk::link_entries(&anchor_address, &address, "motorcycle_link_to", "")?;

        Ok(anchor_address)
    }

    #[zome_fn("hc_public")]
    fn get_motorcycle(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }

}
