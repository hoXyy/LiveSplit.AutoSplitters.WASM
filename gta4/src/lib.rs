#![no_std]
extern crate alloc;

#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

use alloc::{format, string::String, vec::Vec};
use asr::{
    future::next_tick,
    settings::Gui,
    timer::{self, TimerState},
    Address, Process,
};
use asr_watcher::Watcher;

asr::async_main!(stable);
asr::panic_handler!();

#[derive(Gui)]
struct Settings {
    /// Start timer automatically
    #[default = true]
    start_timer: bool,
    /// Reset timer automatically
    #[default = true]
    reset_timer: bool,
    /// Split on mission pass
    #[default = true]
    missions: bool,
    /// Split on stunt jump completion
    #[default = true]
    stunts: bool,
    /// Split on Most Wanted target kill
    #[default = true]
    most_wanted: bool,
    /// Split on flying rat kill
    #[default = true]
    flying_rats: bool,
}

struct MemoryAddresses {
    loading: u64,
    missions_passed: u64,
    missions_attempted: u64,
    stunts: u64,
    most_wanted: u64,
    flying_rats: u64,
    white_loading_screen: u64,
    video_editor: u64,
}

// Only Patch 4 supported for now
const MEMORY_ADDRESSES: MemoryAddresses = MemoryAddresses {
    loading: 0xC07A0C,
    white_loading_screen: 0x01223EA8,
    missions_passed: 0x00C61420,
    missions_attempted: 0x00C61428,
    stunts: 0x00C61464,
    most_wanted: 0x00C615CC,
    flying_rats: 0x00C615D0,
    video_editor: 0xBCCDE0,
};

#[derive(Default, Copy, Clone)]
struct GameState {
    loading: Watcher<u32>,
    missions_passed: Watcher<i32>,
    missions_attempted: Watcher<i32>,
    stunts: Watcher<i32>,
    most_wanted: Watcher<i32>,
    flying_rats: Watcher<i32>,
    white_loading_screen: Watcher<u32>,
    video_editor: Watcher<i32>,
}

impl GameState {
    fn read<T: Copy + Default + bytemuck::CheckedBitPattern>(
        process: &Process,
        base_address: Address,
        offsets: &[u64],
    ) -> Result<T, asr::Error> {
        process.read_pointer_path::<T>(base_address, asr::PointerSize::Bit32, offsets)
    }

    fn update(&mut self, process: &Process, base_address: Address) {
        self.loading.update_from(Self::read::<u32>(
            process,
            base_address,
            &[MEMORY_ADDRESSES.loading],
        ));

        self.missions_passed.update_from(Self::read::<i32>(
            process,
            base_address,
            &[MEMORY_ADDRESSES.missions_passed, 0x10],
        ));

        self.missions_attempted.update_from(Self::read::<i32>(
            process,
            base_address,
            &[MEMORY_ADDRESSES.missions_attempted, 0x10],
        ));

        self.white_loading_screen.update_from(Self::read::<u32>(
            process,
            base_address,
            &[MEMORY_ADDRESSES.white_loading_screen],
        ));

        self.stunts.update_from(Self::read::<i32>(
            process,
            base_address,
            &[MEMORY_ADDRESSES.stunts, 0x10],
        ));

        self.most_wanted.update_from(Self::read::<i32>(
            process,
            base_address,
            &[MEMORY_ADDRESSES.most_wanted, 0x10],
        ));

        self.flying_rats.update_from(Self::read::<i32>(
            process,
            base_address,
            &[MEMORY_ADDRESSES.flying_rats, 0x10],
        ));

        self.video_editor.update_from(Self::read::<i32>(
            process,
            base_address,
            &[MEMORY_ADDRESSES.video_editor],
        ));
    }
}

async fn main() {
    let mut settings = Settings::register();
    let mut state = GameState::default();
    let mut done_splits: Vec<String> = Vec::new();
    let mut timer_state: TimerState = TimerState::NotRunning;

    loop {
        if timer::state() != timer_state {
            done_splits.clear();
            timer_state = timer::state();
            asr::print_message("Cleaning done splits list");
        }

        // Seems to be a Linux/Wine quirk
        let process = Process::wait_attach("LaunchGTAIV.exe").await;
        process
            .until_closes(async {
                if let Ok(base_address) = process.get_module_address("GTAIV.exe") {
                    loop {
                        settings.update();
                        state.update(&process, base_address);

                        // Loading check
                        match state.loading.current == 0 || state.video_editor.current == 256 {
                            true => {
                                timer::pause_game_time();
                            }
                            false => {
                                timer::resume_game_time();
                            }
                        }

                        let start_check: bool = state.white_loading_screen.current == 0
                            && state.white_loading_screen.old != 0
                            && state.loading.current == 0;

                        let missions_check: bool = state.missions_attempted.current == 0;

                        if settings.reset_timer {
                            if start_check
                                && missions_check
                                && timer::state() == TimerState::Running
                            {
                                timer::reset();
                            }
                        }

                        if settings.start_timer {
                            if start_check
                                && missions_check
                                && timer::state() == TimerState::NotRunning
                            {
                                timer::start();
                            }
                        }

                        if timer::state() == TimerState::Running {
                            if settings.missions {
                                if state.missions_passed.current == state.missions_passed.old + 1 {
                                    let key = format!("mission {}", state.missions_passed.current);
                                    if !done_splits.contains(&key) {
                                        asr::print_message(&key);
                                        timer::split();
                                        done_splits.push(key);
                                    }
                                }
                            }

                            if settings.stunts {
                                if state.stunts.current == state.stunts.old + 1 {
                                    let key = format!("stunt {}", state.stunts.current);
                                    if !done_splits.contains(&key) {
                                        timer::split();
                                        done_splits.push(key);
                                    }
                                }
                            }

                            if settings.flying_rats {
                                if state.flying_rats.current == state.flying_rats.old + 1 {
                                    let key = format!("rat {}", state.flying_rats.current);
                                    if !done_splits.contains(&key) {
                                        timer::split();
                                        done_splits.push(key);
                                    }
                                }
                            }

                            if settings.most_wanted {
                                if state.most_wanted.current == state.most_wanted.old + 1 {
                                    let key = format!("most_wanted {}", state.most_wanted.current);
                                    if !done_splits.contains(&key) {
                                        timer::split();
                                        done_splits.push(key);
                                    }
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
