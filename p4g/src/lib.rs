#![no_std]
extern crate alloc;
use asr::{future::next_tick, timer, Process};

#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

asr::async_main!(stable);
asr::panic_handler!();

// only latest Steam supported for now
const LOADING_ADDRESS: u64 = 0x51BCD12;

async fn main() {
    loop {
        timer::pause_game_time();

        let process = Process::wait_attach("P4G.exe").await;
        timer::resume_game_time();

        process
            .until_closes(async {
                if let Ok(base_address) = process.get_module_address("P4G.exe") {
                    loop {
                        if let Ok(loading) = process.read_pointer_path::<u16>(
                            base_address,
                            asr::PointerSize::Bit32,
                            &[LOADING_ADDRESS],
                        ) {
                            match loading != 1 {
                                true => {
                                    timer::pause_game_time();
                                }
                                false => {
                                    timer::resume_game_time();
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
