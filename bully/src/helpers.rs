use alloc::{format, string::String};

pub fn get_gnome_key(index: u64) -> String {
    format!("GNOME_{index}")
}

pub fn get_errand_key(index: u64) -> String {
    format!("ER_{index}")
}
