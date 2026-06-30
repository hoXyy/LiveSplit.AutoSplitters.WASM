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

        let (process_name, process) = retry(|| {
            ["P3P.exe", "p3p_sln_DT_m.exe"]
                .into_iter()
                .find_map(|name| Process::attach(name).map(|proc| (name, proc)))
        })
        .await;

        timer::resume_game_time();

        let loading_address: u64 = if process_name == "P4G.exe" {
            0x9CF134
        } else {
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
