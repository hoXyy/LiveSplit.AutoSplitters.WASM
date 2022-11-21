use asr::timer;
use std::sync::Mutex;

pub mod game;
use game::{GameProcess, Variables};

static GAME_PROCESS: Mutex<Option<GameProcess>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn update() {
    let mut mutex = GAME_PROCESS.lock().unwrap();

    if mutex.is_none() {
        // (Re)connect to the game and unpause game time
        *mutex = GameProcess::connect("P4G");
        timer::resume_game_time();
    } else {
        let game = mutex.as_mut().unwrap();

        // Make sure we're still connected to the game, pause game time if not
        if !game.process.is_open() {
            *mutex = None;
            timer::pause_game_time();
            return;
        }

        let vars = match game.state.update(&mut game.process) {
            Some(v) => v,
            None => {
                asr::print_message("Error updating state!");
                return;
            }
        };

        handle_load(&vars);
    }
}

fn handle_load(vars: &Variables) {
    if vars.loading.current != 1 {
        timer::pause_game_time();
    } else {
        timer::resume_game_time();
    }
}
