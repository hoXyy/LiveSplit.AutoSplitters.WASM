#![no_std]
extern crate alloc;

pub mod helpers;
pub mod missions;
pub mod settings;
pub mod split_guard;
pub mod watchers;

#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

use crate::{
    helpers::get_gnome_key,
    missions::{GNOMES_COUNT, MISSIONS},
    settings::{register_settings, setting_enabled},
    split_guard::SplitGuard,
    watchers::Watchers,
};

use alloc::string::ToString;
use asr::{
    future::next_tick,
    settings::Map,
    time::Duration,
    timer::{self, TimerState},
    Process,
};

asr::async_main!(stable);
asr::panic_handler!();

const PROCESS_NAME: &str = "Bully.exe";

async fn main() {
    register_settings();

    loop {
        let process = Process::wait_attach(PROCESS_NAME).await;
        process
            .until_closes(async {
                if let Ok(base_address) = process.get_module_address(PROCESS_NAME) {
                    let mut watchers = Watchers::new();
                    let mut split_guard = SplitGuard::new();
                    let settings_map = Map::load();

                    loop {
                        watchers.update(&process, base_address);

                        if let Some(igt_pair) = &watchers.igt.pair {
                            timer::pause_game_time(); // fixes game time glitching when paused
                            timer::set_game_time(Duration::milliseconds(igt_pair.current.into()));

                            if igt_pair.old == 0 && igt_pair.current != 0 {
                                split_guard.clear();
                                split_guard.refresh_finished_missions_list(&watchers);
                            }
                        }

                        if setting_enabled(&settings_map, "timer_reset", true) {
                            if let Some(m1_state_pair) = &watchers.m1_state.pair {
                                if m1_state_pair.current == 17 && m1_state_pair.old == 0 {
                                    timer::reset();
                                }
                            }
                        }

                        if setting_enabled(&settings_map, "timer_start", true) {
                            if let Some(m1_state_pair) = &watchers.m1_state.pair {
                                if m1_state_pair.current == 17 && m1_state_pair.old == 0 {
                                    timer::start();
                                    split_guard.clear();
                                }
                            }
                        }

                        for &(_, _, missions) in MISSIONS {
                            for &(key, _, _, _) in missions {
                                if let Some(watcher) = watchers.missions.get(key) {
                                    if let Some(pair) = watcher.pair {
                                        timer::set_variable(key, &pair.current.to_string());
                                    }
                                }
                            }
                        }

                        if timer::state() == TimerState::Running {
                            for &(_, _, missions) in MISSIONS {
                                for &(key, _, _, default_setting) in missions {
                                    if let Some(mission_watcher) = watchers.missions.get(key) {
                                        if let Some(mission_pair) = mission_watcher.pair {
                                            if key == "M_2_03R" {
                                                if setting_enabled(
                                                    &settings_map,
                                                    "M_2_03R1",
                                                    default_setting,
                                                ) && !split_guard
                                                    .missions_done
                                                    .contains_key("M_2_03R1")
                                                {
                                                    if mission_pair.current == 1
                                                        && mission_pair.old == 0
                                                    {
                                                        timer::split();
                                                        split_guard
                                                            .missions_done
                                                            .insert("M_2_03R1".to_string(), true);
                                                    }
                                                }

                                                if setting_enabled(
                                                    &settings_map,
                                                    "M_2_03R2",
                                                    default_setting,
                                                ) && !split_guard
                                                    .missions_done
                                                    .contains_key("M_2_03R2")
                                                {
                                                    if mission_pair.current == 2
                                                        && mission_pair.old == 1
                                                    {
                                                        timer::split();
                                                        split_guard
                                                            .missions_done
                                                            .insert("M_2_03R2".to_string(), true);
                                                    }
                                                }

                                                if setting_enabled(
                                                    &settings_map,
                                                    "M_2_03R3",
                                                    default_setting,
                                                ) && !split_guard
                                                    .missions_done
                                                    .contains_key("M_2_03R3")
                                                {
                                                    if mission_pair.current == 3
                                                        && mission_pair.old == 2
                                                    {
                                                        timer::split();
                                                        split_guard
                                                            .missions_done
                                                            .insert("M_2_03R3".to_string(), true);
                                                    }
                                                }
                                            }

                                            if setting_enabled(&settings_map, key, default_setting)
                                                && !split_guard.missions_done.contains_key(key)
                                            {
                                                if mission_pair.increased() {
                                                    timer::split();
                                                    split_guard
                                                        .missions_done
                                                        .insert(key.to_string(), true);
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            let mut collected_gnome_count: u8 = 0;
                            for i in 0..GNOMES_COUNT {
                                let key = get_gnome_key(i);
                                if let Some(gnome_watcher) = watchers.gnomes.get(&key) {
                                    if let Some(pair) = gnome_watcher.pair {
                                        if pair.increased() {
                                            if setting_enabled(&settings_map, &key, false)
                                                && !split_guard.gnomes_done.contains_key(&key)
                                            {
                                                timer::split();
                                                split_guard.gnomes_done.insert(key, true);
                                            }
                                            collected_gnome_count = collected_gnome_count + 1;
                                        }
                                    }
                                }
                            }

                            if setting_enabled(&settings_map, "all_gnomes", false)
                                && !split_guard.gnomes_done.contains_key("all_gnomes")
                            {
                                if collected_gnome_count == 25 {
                                    timer::split();
                                    split_guard
                                        .gnomes_done
                                        .insert("all_gnomes".to_string(), true);
                                }
                            }
                        }

                        next_tick().await;
                    }
                }
            })
            .await;
    }
}
