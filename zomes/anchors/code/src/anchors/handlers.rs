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
