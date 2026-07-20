#![no_std]
extern crate alloc;

pub mod helpers;
pub mod missions;
pub mod settings;
pub mod split_guard;
pub mod version;
pub mod watchers;

#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

use alloc::{
    format,
    string::{String, ToString},
};
use asr::{
    future::next_tick,
    settings::Map,
    timer::{self, TimerState},
    Process,
};

use crate::{
    helpers::mission_start_text,
    missions::{COLLECTIBLES, MISSIONS},
    settings::{register_settings, setting_enabled},
    split_guard::{SplitGuard, MAX_COLLECTIBLE},
    version::Version,
    watchers::Watchers,
};

asr::async_main!(stable);
asr::panic_handler!();

const PROCESS_NAME: &str = "gta3.exe";

async fn main() {
    register_settings();

    loop {
        let process = Process::wait_attach(PROCESS_NAME).await;
        process
            .until_closes(async {
                if let Ok(base_address) = process.get_module_address(PROCESS_NAME) {
                    if let Some(version) = Version::detect(&process, base_address) {
                        let mut watchers = Watchers::new(version);
                        let mut split_guard = SplitGuard::new();

                        // JP shifts the gameState sentinel values by 4.
                        let gs_shift: u32 = if version == Version::Japanese { 4 } else { 0 };

                        loop {
                            watchers.update(&process, base_address);

                            let settings_map = Map::load();
                            let timer_state = timer::state();

                            if let Some(gs) = watchers.game_state.pair() {
                                if setting_enabled(&settings_map, "timer_start", true)
                                    && gs.old == 8 + gs_shift
                                    && gs.current == 9 + gs_shift
                                    && timer_state == TimerState::NotRunning
                                {
                                    timer::start();
                                    split_guard.clear();
                                }

                                if setting_enabled(&settings_map, "timer_reset", true)
                                    && gs.old == 9 + gs_shift
                                    && gs.current == 8 + gs_shift
                                {
                                    timer::reset();
                                }
                            }

                            if timer_state == TimerState::Running {
                                for (i, &(complete_key, _, _)) in MISSIONS.iter().enumerate() {
                                    if split_guard.missions_complete[i] {
                                        continue;
                                    }
                                    if !setting_enabled(&settings_map, complete_key, true) {
                                        continue;
                                    }
                                    if let Some(p) = watchers.missions[complete_key].pair() {
                                        if p.current > p.old {
                                            split_guard.missions_complete[i] = true;
                                            timer::split();
                                        }
                                    }
                                }

                                if let Some(text_pair) = watchers.mission_text.pair() {
                                    if text_pair.current != text_pair.old {
                                        let current_text =
                                            String::from_utf16(text_pair.current.as_slice())
                                                .unwrap();

                                        for (i, &(complete_key, _, _)) in
                                            MISSIONS.iter().enumerate()
                                        {
                                            let start_key = format!("{complete_key}_start");

                                            if split_guard.missions_start[i] {
                                                continue;
                                            }
                                            if !setting_enabled(
                                                &settings_map,
                                                &start_key.to_string(),
                                                false,
                                            ) {
                                                continue;
                                            }

                                            if let Some(expected) =
                                                mission_start_text(complete_key, version)
                                            {
                                                let matches = if version == Version::Japanese {
                                                    // JP: bare uppercase, e.g. "LUIGI'S GIRLS"
                                                    current_text == expected
                                                } else {
                                                    let quoted = format!("'{expected}'");
                                                    current_text == quoted.as_str()
                                                };

                                                if matches {
                                                    split_guard.missions_start[i] = true;
                                                    timer::split();
                                                }
                                            }
                                        }
                                    }
                                }

                                for (i, &(key, _, _, max)) in COLLECTIBLES.iter().enumerate() {
                                    if let Some(p) = watchers.collectibles[key].pair() {
                                        if p.current <= p.old {
                                            continue;
                                        }

                                        let key_all = format!("{key}_all");
                                        let key_each = format!("{key}_each");

                                        if setting_enabled(&settings_map, key_all.as_str(), false)
                                            && !split_guard.collectibles_all[i]
                                            && p.current == max
                                            && p.old == max - 1
                                        {
                                            split_guard.collectibles_all[i] = true;
                                            timer::split();
                                        }

                                        if setting_enabled(&settings_map, key_each.as_str(), false)
                                        {
                                            let idx = (p.current as usize).saturating_sub(1);
                                            if idx < MAX_COLLECTIBLE
                                                && !split_guard.collectibles_each[i][idx]
                                            {
                                                split_guard.collectibles_each[i][idx] = true;
                                                timer::split();
                                            }
                                        }
                                    }
                                }

                                if setting_enabled(&settings_map, "btg_final_split", true) {
                                    if let (Some(hp), Some(tm)) =
                                        (watchers.te_helipad.pair(), watchers.te_timer.pair())
                                    {
                                        if hp.current == 1 && tm.current != tm.old {
                                            timer::split();
                                        }
                                    }
                                }

                                if setting_enabled(&settings_map, "hundo_final_split", false) {
                                    if let Some(progress_made) = watchers.progress_made.pair() {
                                        if progress_made.current == 154 && progress_made.old != 154
                                        {
                                            timer::split();
                                        }
                                    }
                                }
                            }

                            next_tick().await;
                        }
                    }
                }
            })
            .await;
    }
}
