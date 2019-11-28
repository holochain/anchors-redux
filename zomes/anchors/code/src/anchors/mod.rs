use hdk::{
    entry_definition::ValidatingEntryType,
};
use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
};

use hdk::holochain_json_api::{
    json::JsonString,
    error::JsonError
};

use crate::{
    ROOT_ANCHOR_ENTRY,
    ROOT_ANCHOR_LINK_TO,
    ANCHOR_ENTRY
};

pub mod handlers;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Anchor {
    anchor_type: String,
    anchor_text: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct RootAnchor {
    anchor_type: String,
}

pub fn root_anchor_definition() -> ValidatingEntryType {
    entry!(
        name: ROOT_ANCHOR_ENTRY,
        description: "All other anchors are linked from the root anchor so we can list all the anchors.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<RootAnchor>| {
            Ok(())
        },
        links: [
            to!(
                ANCHOR_ENTRY,
                link_type: ROOT_ANCHOR_LINK_TO,

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

pub fn anchor_definition() -> ValidatingEntryType {
    entry!(
        name: ANCHOR_ENTRY,
        description: "Anchors are used as the base for links so linked entries can be found with a text search.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Anchor>| {
            Ok(())
        }
    )
}
