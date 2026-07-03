use alloc::format;
use asr::settings::{
    gui::{add_bool, add_title, set_tooltip},
    Map,
};

use crate::{
    helpers::get_gnome_key,
    missions::{ERRANDS_COUNT, GNOMES_COUNT, MISSIONS},
};

pub fn register_settings() {
    // Timer Controls
    add_title("timer_controls", "Timer Controls", 0);
    add_bool("timer_start", "Start timer automatically", true);
    add_bool("timer_reset", "Reset timer automatically", true);

    // Missions
    add_title("missions", "Missions", 0);
    for &(key, title, missions) in MISSIONS {
        let header_level = if key == "misc" { 0 } else { 1 };

        add_title(key, title, header_level);

        if key == "M_PR" {
            // Paper Routes have custom logic so need to add these settings manually
            add_bool("M_PR_01", "Intro", false);
            add_bool("M_PR_D1", "10 Clients", false);
            add_bool("M_PR_D2", "14 Clients", false);
            add_bool("M_PR_D3", "19 Clients", false);
            add_bool("M_PR_D4", "24 Clients", false);
            continue;
        }

        for &(key, mission_title, _, default) in missions {
            if key == "M_2_03R" {
                add_bool("M_2_03R1", "Chad (Prep Challenge)", false);
                add_bool("M_2_03R2", "Justin (Prep Challenge)", false);
                add_bool("M_2_03R3", "Parker (Prep Challenge)", false);
                set_tooltip("M_2_03R1", "Additional split for Chad.");
                set_tooltip("M_2_03R2", "Additional split for Justin.");
                set_tooltip("M_2_03R3", "Additional split for Parker.");
                continue;
            }

            add_bool(key, mission_title, default);
        }
    }

    // Collectibles
    add_title("collectibles", "Collectibles", 0);
    add_bool("all_gnomes", "All Gnomes", false);

    add_title("each_gnome", "Gnomes", 1);
    for id in 0..GNOMES_COUNT {
        let setting_key = get_gnome_key(id);
        let setting_title = format!("Gnome {}", id + 1);
        add_bool(&setting_key, &setting_title, false);
    }

    set_tooltip("M_1_01a", "Additional split after going to the Principal.");
    set_tooltip("M_1_01b", "Additional split when entering Boys Dorm.");
    set_tooltip(
        "C_C_02",
        "Splits after the Chapter 2 introduction cutscene.",
    );
    set_tooltip(
        "C_C_03",
        "Splits after the Chapter 3 introduction cutscene.",
    );
    set_tooltip(
        "C_C_04",
        "Splits after the Chapter 4 introduction cutscene.",
    );
    set_tooltip(
        "C_C_05",
        "Splits after the Chapter 5 introduction cutscene.",
    );
    set_tooltip(
        "C_C_C",
        "Splits after Credits where you would normally pause at Jimmy's room.",
    );
}

pub fn setting_enabled(map: &Map, key: &str, default: bool) -> bool {
    map.get(key).and_then(|v| v.get_bool()).unwrap_or(default)
}
