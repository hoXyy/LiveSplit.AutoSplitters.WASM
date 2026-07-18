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
    let mut watchers = Watchers::new();

    loop {
        let process = Process::wait_attach(PROCESS_NAME).await;
        process
            .until_closes(async {
                if let Ok(base_address) = process.get_module_address(PROCESS_NAME) {
                    if let Some(version) = Version::detect(&process, base_address) {
                        loop {
                            settings.update();
                            watchers.update(&process, base_address, version);
                            let timer_state = timer::state();

                            if let (Some(save_load), Some(cutscene_load)) =
                                (&watchers.save_load.pair, &watchers.cutscene_load.pair)
                            {
                                if cutscene_load.current == 0 || save_load.current == 0 {
                                    timer::pause_game_time();
                                } else {
                                    timer::resume_game_time();
                                }
                            }

                            if timer_state == TimerState::NotRunning {
                                if settings.timer_start {
                                    if let (Some(cutscene), Some(start_flag)) =
                                        (&watchers.cutscene.pair, &watchers.start_flag.pair)
                                    {
                                        if let Ok(current_cutscene) =
                                            cutscene.current.validate_utf8()
                                        {
                                            if current_cutscene == "TSSP01-01.cscx"
                                                && start_flag.current == 1
                                                && start_flag.old != start_flag.current
                                            {
                                                timer::start();
                                            }
                                        }
                                    }
                                }
                            }

                            if timer_state == TimerState::Running {
                                if settings.timer_reset {
                                    if let Some(cutscene) = &watchers.cutscene.pair {
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
                                    if let Some(progress_percent) = &watchers.progress_percent.pair
                                    {
                                        if progress_percent.current == 100
                                            && progress_percent.changed()
                                        {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.missions {
                                    if let Some(missions) = &watchers.missions.pair {
                                        if missions.current == missions.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.strongholds {
                                    if let Some(strongholds) = &watchers.strongholds.pair {
                                        if strongholds.current == strongholds.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.tags {
                                    if let Some(tags) = &watchers.tags.pair {
                                        if tags.current == tags.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.cd {
                                    if let Some(cd) = &watchers.cd.pair {
                                        if cd.current == cd.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.jumps {
                                    if let Some(jumps) = &watchers.jumps.pair {
                                        if jumps.current == jumps.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.barnstorming {
                                    if let Some(barnstorming) = &watchers.barnstorming.pair {
                                        if barnstorming.current == barnstorming.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.chop_shop {
                                    if let Some(chop_shop) = &watchers.chop_shop.pair {
                                        if chop_shop.current == chop_shop.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.crowd_control {
                                    if let Some(crowd_control) = &watchers.crowd_control.pair {
                                        if crowd_control.current == crowd_control.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.derby {
                                    if let Some(derby) = &watchers.derby.pair {
                                        if derby.current == derby.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.escort {
                                    if let Some(escort) = &watchers.escort.pair {
                                        if escort.current == escort.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.fight_club {
                                    if let Some(fight_club) = &watchers.fight_club.pair {
                                        if fight_club.current == fight_club.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.fuzz {
                                    if let Some(fuzz) = &watchers.fuzz.pair {
                                        if fuzz.current == fuzz.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.heli_assault {
                                    if let Some(heli_assault) = &watchers.heli_assault.pair {
                                        if heli_assault.current == heli_assault.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.hitman {
                                    if let Some(hitman) = &watchers.hitman.pair {
                                        if hitman.current == hitman.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.fraud {
                                    if let Some(fraud) = &watchers.fraud.pair {
                                        if fraud.current == fraud.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.mayhem {
                                    if let Some(mayhem) = &watchers.mayhem.pair {
                                        if mayhem.current == mayhem.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.races {
                                    if let Some(races) = &watchers.races.pair {
                                        if races.current == races.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.septic {
                                    if let Some(septic) = &watchers.septic.pair {
                                        if septic.current == septic.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.snatch {
                                    if let Some(snatch) = &watchers.snatch.pair {
                                        if snatch.current == snatch.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.trafficking {
                                    if let Some(trafficking) = &watchers.trafficking.pair {
                                        if trafficking.current == trafficking.old + 1 {
                                            timer::split();
                                        }
                                    }
                                }

                                if settings.trail_blazing {
                                    if let Some(trail_blazing) = &watchers.trail_blazing.pair {
                                        if trail_blazing.current == trail_blazing.old + 1 {
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
