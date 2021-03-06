//! Fuctions and Structs for dealing with /etc/shadow

use std::path::Path;
use std::num::ParseIntError;
use libc::types::os::arch::c95::c_long;
use libc::types::os::arch::c95::c_ulong;
use entries::{Entries,Entry};


/// An entry from /etc/shadow
#[derive(Debug, PartialEq, PartialOrd)]
pub struct ShadowEntry {
    /// Login name
    pub name: String,

    /// Encrypted password
    pub passwd: String,

    /// Date of last change (measured in days since 1970-01-01 00:00:00 +0000 (UTC))
    pub last_change: c_long,

    /// Min number of days between changes
    pub min: c_long,

    /// Max number of days between changes
    pub max: c_long,

    /// Number of days before password expires to warn user to change it
    pub warning: c_long,

    /// Number of days after password expires until account is disabled
    pub inactivity: c_long,

    /// Date when account expires (measured
    /// in days since 1970-01-01 00:00:00 +0000 (UTC))
    pub expires: c_long,

    /// Reserved
    pub flag: c_ulong,
}


impl Entry for ShadowEntry {
    fn from_line(line: &str) -> Result<ShadowEntry, ParseIntError> {

        let parts: Vec<&str> = line.split(":").map(|part| part.trim()).collect();

        Ok(ShadowEntry {
            name: parts[0].to_string(),
            passwd: parts[1].to_string(),
            last_change: parts[2].parse().unwrap_or(-1),
            min: parts[3].parse().unwrap_or(-1),
            max: parts[4].parse().unwrap_or(-1),
            warning: parts[5].parse().unwrap_or(-1),
            inactivity: parts[6].parse().unwrap_or(-1),
            expires: parts[7].parse().unwrap_or(-1),
            flag: parts[8].parse().unwrap_or(0),
        })
    }
}


/// Return a [`ShadowEntry`](struct.ShadowEntry.html)
/// for a given `name` and `&Path`
pub fn get_entry_by_name_from_path(path: &Path, name: &str) -> Option<ShadowEntry> {
    Entries::<ShadowEntry>::new(path).find(|x| x.name == name)
}


/// Return a [`ShadowEntry`](struct.ShadowEntry.html)
/// for a given `name` from `/etc/shadow`
pub fn get_entry_by_name(name: &str) -> Option<ShadowEntry> {
    get_entry_by_name_from_path(&Path::new("/etc/shadow"), name)
}


/// Return a `Vec<`[`ShadowEntry`](struct.ShadowEntry.html)`>` containing all
/// [`ShadowEntry`](struct.ShadowEntry.html)'s for a given `&Path`
pub fn get_all_entries_from_path(path: &Path) -> Vec<ShadowEntry> {
    Entries::new(path).collect()
}


/// Return a `Vec<`[`ShadowEntry`](struct.ShadowEntry.html)`>` containing all
/// [`ShadowEntry`](struct.ShadowEntry.html)'s from `/etc/shadow`
pub fn get_all_entries() -> Vec<ShadowEntry> {
    get_all_entries_from_path(&Path::new("/etc/shadow"))
}
