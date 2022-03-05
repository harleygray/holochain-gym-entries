use hdk::prelude::holo_hash::{EntryHashB64, HeaderHashB64};
use hdk::prelude::*;

entry_defs![SnackingLog::entry_def()];

#[hdk_entry(id = "SnackingLog")]
pub struct SnackingLog(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HeaderAndEntryHash {
    entry_hash: EntryHashB64,
    header_hash: HeaderHashB64,
}

#[hdk_extern]
pub fn register_snacking(input: SnackingLog) -> ExternResult<HeaderAndEntryHash> {
    // input needs to be referenced as it is used twice
    let header = create_entry(&input)?;
    let entry = hash_entry(&input)?;

    // from!!! this was the part i was missing. how does this work? 
    // the `From` trait is derived in the HoloHashB64 definition (derive_more::From)
    // traits allow a set of methods to be defined across different types
    // so, the from() method is implemented for EntryHashB64 from an EntryHash
    let snack_hashes = HeaderAndEntryHash {
        entry_hash: EntryHashB64::from(entry),
        header_hash: HeaderHashB64::from(header),
    };
    Ok(snack_hashes)
}

#[hdk_extern]
pub fn get_by_header_hash(header_hash: HeaderHash) -> ExternResult<SnackingLog> {

    // tick! got this part. got stuck with how to unwrap the Details type. where i got stuck was casting this as an Element, since
    // it is a headerhash passed in. it will always get the element
    let element: Element = get(HeaderHash::from(header_hash), GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("unable to retrieve")))?;

    // entry() here seems to be referring to the entry field of Element struct?
    // the Element definition has an implementation for a function called 
    // entry() which access the entry portion of the element as ElementEntry
    // ElementEntry then has an implementation for the to_app_option() method
    // which deserialises the app entry if it exists
    let option: Option<SnackingLog> = element.entry().to_app_option()?;

    // unwrap the option
    let snack_log: SnackingLog = 
        option.ok_or(WasmError::Guest(String::from("no log inside option")))?;


    Ok(snack_log)

}

#[hdk_extern]
pub fn get_by_entry_hash(entry_hash: EntryHash) -> ExternResult<SnackingLog> {

    // get details
    let element: Element = get(EntryHash::from(entry_hash), GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("unable to locate via entryhash")))?;

    let option: Option<SnackingLog> = element.entry().to_app_option()?;

    let snack_log: SnackingLog = 
        option.ok_or(WasmError::Guest(String::from("cannot retrieve a snacking log from retrieved element/entry")))?;

    Ok(snack_log)
}

#[hdk_extern]
pub fn get_all_headers_from_content(input: SnackingLog) -> ExternResult<Vec<SignedHeaderHashed>> {
    let entry = hash_entry(&input)?;

    // Details for an entry hash return:
    //     all creates, updates and delete elements that reference that entry hash
    //     all update and delete elements that reference the elements that reference the entry hash
    let details = match get_details(entry, GetOptions::default())? {
        Some(successful_output) => successful_output,
        None => return Ok(vec![]), 
    };

    // Since an EntryHash is passed in, Details will be the Entry enum variant
    let entry_option: Option<EntryDetails> = match details {
            Details::Entry(entry_details) => Some(entry_details),
            Details::Element(_) => None,
    };

    match entry_option {
        Some(entry) => Ok(entry.headers),
        None => Ok(vec![]),
    }
}
