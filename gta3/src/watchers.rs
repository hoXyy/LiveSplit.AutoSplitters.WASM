use crate::{version::Version, COLLECTIBLES, MISSIONS};
use asr::{string::ArrayWString, watcher::Watcher, Address, Process};
use core::array::from_fn;

pub struct Watchers {
    pub missions: [Watcher<u32>; MISSIONS.len()],
    pub collectibles: [Watcher<u32>; COLLECTIBLES.len()],
    pub game_state: Watcher<u32>,
    pub mission_text: Watcher<ArrayWString<128>>,
    pub progress_made: Watcher<u32>,
    pub te_helipad: Watcher<u8>,
    pub te_timer: Watcher<u32>,
}

impl Default for Watchers {
    fn default() -> Self {
        Self::new()
    }
}

impl Watchers {
    pub fn new() -> Self {
        Self {
            missions: from_fn(|_| Watcher::new()),
            collectibles: from_fn(|_| Watcher::new()),
            game_state: Watcher::new(),
            mission_text: Watcher::new(),
            progress_made: Watcher::new(),
            te_helipad: Watcher::new(),
            te_timer: Watcher::new(),
        }
    }

    pub fn update(&mut self, process: &Process, base: Address, version: Version) {
        let off = version.offset();

        let addr = |raw: u64| Address::new((raw as i64 + off) as u64 + base.value());

        for (i, &(_, _, address)) in MISSIONS.iter().enumerate() {
            self.missions[i].update(process.read(addr(address)).ok());
        }
        for (i, &(_, _, address, _)) in COLLECTIBLES.iter().enumerate() {
            self.collectibles[i].update(process.read(addr(address)).ok());
        }

        // game_state and progress_made have separate JP addresses entirely.
        let game_state_addr = if version == Version::Japanese {
            Address::new(base.value() + 0x50387C)
        } else {
            addr(0x505A2C)
        };
        let progress_addr = if version == Version::Japanese {
            Address::new(base.value() + 0x50436C)
        } else {
            addr(0x50651C)
        };
        let mission_text_addr = if version == Version::Japanese {
            Address::new(base.value() + 0x272AE8)
        } else {
            addr(0x274F20)
        };

        self.game_state.update(process.read(game_state_addr).ok());
        self.progress_made.update(process.read(progress_addr).ok());
        self.mission_text
            .update(process.read(mission_text_addr).ok());
        self.te_helipad.update(process.read(addr(0x35F6B8)).ok());
        self.te_timer.update(process.read(addr(0x35BA2C)).ok());
    }
}
