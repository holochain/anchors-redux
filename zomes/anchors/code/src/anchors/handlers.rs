use hdk::{
    self,
    error::ZomeApiResult,
    holochain_core_types::{
        entry::Entry,
    },
    holochain_persistence_api::{
        cas::content::{Address, AddressableContent},
    },
};

use crate::anchors::{
    RootAnchor,
    Anchor
};
use crate::{
    ROOT_ANCHOR_ENTRY,
    ROOT_ANCHOR_LINK_TO,
    ANCHOR_ENTRY
};
use hdk::prelude::*;
// Will only create a new root anchor if the Agent is not synched or its first hardTimeout
// will accumulate headers for each agent that adds.
// Ideally it would be cool to be able to linkn to the root_anchor_entry.address() and it not be an entry.
pub (crate) fn root_anchor() -> ZomeApiResult<Address> {
    let root_anchor_entry = Entry::App(
        ROOT_ANCHOR_ENTRY.into(),
        RootAnchor {anchor_type: "root_anchor".into()}.into()
    );
    let root_anchor_entry_address = root_anchor_entry.address();
    if hdk::get_entry(&root_anchor_entry_address)?.is_none() {
        Ok(hdk::commit_entry(&root_anchor_entry)?)
    } else {
        Ok(root_anchor_entry_address)
    }
}

pub(crate) fn handle_create_anchor(anchor_type: String, anchor_text: String) -> ZomeApiResult<Address> {
    let anchor_entry = Entry::App(
        ANCHOR_ENTRY.into(),
        Anchor {
            anchor_type,
            anchor_text
        }.into()
    );
    let anchor_address = hdk::commit_entry(&anchor_entry)?;
    hdk::link_entries(&root_anchor().unwrap(), &anchor_address, ROOT_ANCHOR_LINK_TO, "")?;
    Ok(anchor_address)
}

pub(crate) fn handle_get_anchor(anchor_address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&anchor_address)
}

pub(crate) fn handle_get_anchors() -> ZomeApiResult<Vec<Address>> {
    let root_anchor_entry = Entry::App(
        ROOT_ANCHOR_ENTRY.into(),
        RootAnchor {anchor_type: "root_anchor".into()}.into()
    );
    let root_anchor_entry_address = root_anchor_entry.address();
    Ok(hdk::get_links(&root_anchor_entry_address, LinkMatch::Exactly(ROOT_ANCHOR_LINK_TO), LinkMatch::Any)?.addresses().to_owned())
}
