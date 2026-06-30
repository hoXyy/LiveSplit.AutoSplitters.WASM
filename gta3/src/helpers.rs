use crate::{missions::MISSION_TEXT, version::Version};

pub fn mission_start_text(key: &str, version: Version) -> Option<&'static str> {
    // JP overrides
    if version == Version::Japanese {
        if key == "turismo" {
            return Some("ROAD RACING");
        }
        if key == "bomb_da_base_act_i" {
            return Some("BOMB DA BASE -ACT 1-");
        }
        if key == "bomb_da_base_act_ii" {
            return Some("BOMB DA BASE -ACT 2-");
        }
    }

    asr::print_message(key);

    // Look up the base text from the table
    MISSION_TEXT
        .iter()
        .find(|&&(k, _)| k == key)
        .map(|&(_, text)| text)
}
