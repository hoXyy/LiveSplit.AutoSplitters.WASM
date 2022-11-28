#![allow(unused_assignments)]
use once_cell::sync::OnceCell;
use asr::timer::{self, TimerState};
use std::sync::Mutex;

pub mod game;
use game::{GameProcess, Variables};

static GAME_PROCESS: Mutex<Option<GameProcess>> = Mutex::new(None);
static CURRENT_CUTSCENE: OnceCell<Mutex<String>> = OnceCell::new();

#[no_mangle]
pub extern "C" fn update() {
    let mut mutex = GAME_PROCESS.lock().unwrap();

    if mutex.is_none() {
        // (Re)connect to the game and unpause game time
        *mutex = GameProcess::connect("SR2_pc");
    } else {
        let game = mutex.as_mut().unwrap();

        // Make sure we're still connected to the game, pause game time if not
        if !game.process.is_open() {
            *mutex = None;
            return;
        }

        let vars = match game.state.update(&mut game.process) {
            Some(v) => v,
            None => {
                asr::print_message("Error updating state!");
                return;
            }
        };

        if let Some(cutscene) = vars.cutscene {
            let cutscene_text = Variables::get_as_string(&cutscene.current).unwrap();
            if cutscene_text != "" {
                set_current_cutscene(cutscene_text.to_string());
            }
        }

        if timer::state() == TimerState::Running {
            handle_load(&vars);
            handle_split(&vars);
            handle_reset();
        }

        if timer::state() == TimerState::NotRunning {
            handle_start(&vars);
        }
    }
}

fn handle_load(vars: &Variables) {
    if vars.cutscene_load.current == 0 || vars.save_load.current == 0 {
        timer::pause_game_time();
    } else {
        timer::resume_game_time();
    }
}

fn handle_split(vars: &Variables) {
    if vars.missions.current == vars.missions.old + 1
        || vars.strongholds.current == vars.strongholds.old + 1
        || vars.tags.current == vars.tags.old + 1
        || vars.cds.current == vars.cds.old + 1
        || vars.jumps.current == vars.jumps.old + 1
        || vars.barnstorming.current == vars.barnstorming.old + 1
        || vars.chop_shop.current == vars.chop_shop.old + 1
        || vars.crowd_control.current == vars.crowd_control.old + 1
        || vars.derby.current == vars.derby.old + 1
        || vars.escort.current == vars.escort.old + 1
        || vars.fight_club.current == vars.fight_club.old + 1
        || vars.fuzz.current == vars.fuzz.old + 1
        || vars.heli_assault.current == vars.heli_assault.old + 1
        || vars.hitman.current == vars.hitman.old + 1
        || vars.fraud.current == vars.fraud.old + 1
        || vars.mayhem.current == vars.mayhem.old + 1
        || vars.races.current == vars.races.old + 1
        || vars.septic.current == vars.septic.old + 1
        || vars.snatch.current == vars.snatch.old + 1
        || vars.trafficking.current == vars.trafficking.old + 1
        || vars.trail_blazing.current == vars.trail_blazing.old + 1
    {
        timer::split();
    }

    if vars.completion.current == 100 && vars.completion.current != vars.completion.old {
        timer::split();
    }
}

// TODO: Fix, doesn't work yet
fn handle_start(vars: &Variables) {
    asr::print_message(&get_current_cutscene());
    if get_current_cutscene() == "TSSP01-01.cscx" {
        if vars.start_flag.current == 1 && vars.start_flag.current != vars.start_flag.old {
            timer::start();
        }
    }
}

fn handle_reset() {
    if get_current_cutscene() == "TSSP-INTRO2.cscx" {
        timer::reset();
    }
}

fn ensure_cutscene() -> &'static Mutex<String> {
    CURRENT_CUTSCENE.get_or_init(|| Mutex::new(String::new()))
}

fn get_current_cutscene() -> String {
    ensure_cutscene().lock().unwrap().clone()
}

fn set_current_cutscene(cutscene: String) {
    *ensure_cutscene().lock().unwrap() = cutscene;
}
