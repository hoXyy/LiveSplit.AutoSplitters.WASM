#![no_std]
extern crate alloc;

#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

use asr::{
    future::{next_tick, retry},
    timer, Process,
};

asr::async_main!(stable);
asr::panic_handler!();

async fn main() {
    loop {
        timer::pause_game_time();

        let process = retry(|| {
            ["P3P.exe", "p3p_sln_DT_m.exe"]
                .into_iter()
                .find_map(Process::attach)
        })
        .await;
        timer::resume_game_time();

        let mut process_name = "P3P.exe";

        // can't just get the process name since on linux it's just wine64-preloader
        let loading_address: u64 = if process.get_module_address("p3p_sln_DT_m.exe").is_err() {
            0x9CF134
        } else {
            process_name = "p3p_sln_DT_m.exe";
            0x130AF74
        };

        process
            .until_closes(async {
                if let Ok(base_address) = process.get_module_address(process_name) {
                    loop {
                        if let Ok(loading) = process.read_pointer_path::<u16>(
                            base_address,
                            asr::PointerSize::Bit32,
                            &[loading_address],
                        ) {
                            match loading == 4 {
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
