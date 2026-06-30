use crate::{COLLECTIBLES, MISSIONS};
use alloc::format;
use asr::settings::gui::{add_bool, add_title, set_tooltip};
use asr::settings::Map;

pub fn register_settings() {
    // Timer controls
    add_bool("timer_start", "Start timer automatically", true);
    add_bool("timer_reset", "Reset timer automatically", true);

    // Mission end splits
    add_title("title_missions_complete", "Missions (complete)", 0);
    for &(key, title, _) in MISSIONS {
        add_bool(key, title, true);
    }

    // Mission start splits
    add_title("title_missions_start", "Missions (start)", 0);
    for &(key, title, _) in MISSIONS {
        let key_start = format!("{key}_start");

        add_bool(key_start.as_str(), title, false);
    }

    // Collectibles
    add_title("title_collectibles", "Collectibles", 0);
    for &(key, title, _, _) in COLLECTIBLES {
        let key_each = format!("{key}_each");
        let key_all = format!("{key}_all");

        let desc_each = format!("{title} (each)");
        let desc_all = format!("{title} (all)");

        add_bool(key_all.as_str(), desc_all.as_str(), false);
        add_bool(key_each.as_str(), desc_each.as_str(), false);
    }

    // Final splits
    add_title("final_splits", "Final Splits", 0);
    add_bool("btg_final_split", "Any% Final Split", true);
    set_tooltip(
        "btg_final_split",
        "Splits once you lose control on \"The Exchange\".",
    );

    add_bool("hundo_final_split", "100% Final Split", false);
    set_tooltip(
        "hundo_final_split",
        "Splits once you reach 100% game completion.",
    );
}

pub fn setting_enabled(map: &Map, key: &str, default: bool) -> bool {
    map.get(key).and_then(|v| v.get_bool()).unwrap_or(default)
}
