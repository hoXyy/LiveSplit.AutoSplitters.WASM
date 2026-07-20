#![no_std]
extern crate alloc;

pub mod version;
pub mod watchers;

#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

use asr::{
    future::next_tick,
    settings::{gui::Title, Gui},
    timer::{self, TimerState},
    Process,
};

use crate::{version::Version, watchers::Watchers};

asr::async_main!(stable);
asr::panic_handler!();

const PROCESS_NAME: &str = "SR2_pc.exe";

#[derive(Gui)]
struct Settings {
    /// Timer Control
    _timer: Title,
    /// Start timer automatically
    #[default = true]
    timer_start: bool,
    /// Reset timer automatically
    #[default = true]
    timer_reset: bool,
    /// Main
    _main: Title,
    /// Missions
    #[default = true]
    missions: bool,
    /// Strongholds
    #[default = true]
    strongholds: bool,
    /// 100%
    hundo: bool,
    /// Activities
    _activities: Title,
    /// Chop Shop
    #[default = true]
    chop_shop: bool,
    /// Crowd Control
    #[default = true]
    crowd_control: bool,
    /// Destruction Derby
    #[default = true]
    derby: bool,
    /// Escort
    #[default = true]
    escort: bool,
    /// Fight Club
    #[default = true]
    fight_club: bool,
    /// FUZZ
    #[default = true]
    fuzz: bool,
    /// Heli Assault
    #[default = true]
    heli_assault: bool,
    /// Hitman
    #[default = true]
    hitman: bool,
    /// Insurance Fraud
    #[default = true]
    fraud: bool,
    /// Mayhem
    #[default = true]
    mayhem: bool,
    /// Races
    #[default = true]
    races: bool,
    /// Septic Avenger
    #[default = true]
    septic: bool,
    /// Snatch
    #[default = true]
    snatch: bool,
    /// Trafficking
    #[default = true]
    trafficking: bool,
    /// Trail Blazing
    #[default = true]
    trail_blazing: bool,
    /// Collectibles
    _collectibles: Title,
    /// Tags
    tags: bool,
    /// CDs
    cd: bool,
    /// Stunt Jumps
    jumps: bool,
    /// Barnstorming
    barnstorming: bool,
}

async fn main() {
    let mut settings = Settings::register();

    loop {
        let process = Process::wait_attach(PROCESS_NAME).await;
        process
            .until_closes(async {
                if let Ok(base_address) = process.get_module_address(PROCESS_NAME) {
                    if let Some(version) = Version::detect(&process, base_address) {
                        let mut watchers = Watchers::new(version);

                        loop {
                            settings.update();
                            watchers.update(&process, base_address);
                            let timer_state = timer::state();

                            if let (Some(save_load), Some(cutscene_load)) =
                                (watchers.save_load.pair(), watchers.cutscene_load.pair())
                            {
                                if cutscene_load.current == 0 || save_load.current == 0 {
                                    timer::pause_game_time();
                                } else {
                                    timer::resume_game_time();
                                }
                            }

                            if timer_state == TimerState::NotRunning && settings.timer_start {
                                let current_cutscene = watchers.current_cutscene();

                                if let Some(start_flag) = watchers.start_flag.pair() {
                                    if current_cutscene == "TSSP01-01.cscx"
                                        && start_flag.current == 1
                                        && start_flag.old != start_flag.current
                                    {
                                        timer::start();
                                    }
                                }
                            }

                            if timer_state == TimerState::Running {
                                if settings.timer_reset {
                                    if let Some(cutscene) = watchers.cutscene.pair() {
                                        if cutscene.changed() {
                                            if let Ok(current_cutscene) =
                                                cutscene.current.validate_utf8()
                                            {
                                                if current_cutscene == "TSSP-INTRO2.cscx" {
                                                    timer::reset();
                                                }
                                            }
                                        }
                                    }
                                }

                                if settings.hundo {
                                    if let Some(progress_percent) = watchers.progress_percent.pair()
                                    {
                                        if progress_percent.current == 100
                                            && progress_percent.changed()
                                        {
                                            timer::split();
                                        }
                                    }
                                }

                                let counter_settings = [
                                    (settings.missions, "missions"),
                                    (settings.strongholds, "strongholds"),
                                    (settings.tags, "tags"),
                                    (settings.cd, "cd"),
                                    (settings.jumps, "jumps"),
                                    (settings.barnstorming, "barnstorming"),
                                    (settings.chop_shop, "chop_shop"),
                                    (settings.crowd_control, "crowd_control"),
                                    (settings.derby, "derby"),
                                    (settings.escort, "escort"),
                                    (settings.fight_club, "fight_club"),
                                    (settings.fuzz, "fuzz"),
                                    (settings.heli_assault, "heli_assault"),
                                    (settings.hitman, "hitman"),
                                    (settings.fraud, "fraud"),
                                    (settings.mayhem, "mayhem"),
                                    (settings.races, "races"),
                                    (settings.septic, "septic"),
                                    (settings.snatch, "snatch"),
                                    (settings.trafficking, "trafficking"),
                                    (settings.trail_blazing, "trail_blazing"),
                                ];

                                for (enabled, name) in counter_settings {
                                    if enabled && watchers.counters[name].increased_by(1) {
                                        timer::split();
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
