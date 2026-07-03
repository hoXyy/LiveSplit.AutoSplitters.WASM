use alloc::{format, string::String};

pub fn get_gnome_key(index: u64) -> String {
    format!("gnome_{index}")
}

pub fn get_errand_key(index: u64) -> String {
    format!("errand_{index}")
}
