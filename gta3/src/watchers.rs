use crate::{version::Version, COLLECTIBLES, MISSIONS};
use asr::{string::ArrayWString, Address, Process};
use autosplitter_helpers::{MemoryWatcher, MemoryWatcherMap};

pub struct Watchers {
    pub missions: MemoryWatcherMap<u32>,
    pub collectibles: MemoryWatcherMap<u32>,
    pub game_state: MemoryWatcher<u32>,
    pub mission_text: MemoryWatcher<ArrayWString<128>>,
    pub progress_made: MemoryWatcher<u32>,
    pub te_helipad: MemoryWatcher<u8>,
    pub te_timer: MemoryWatcher<u32>,
}

impl Watchers {
    pub fn new(version: Version) -> Self {
        let off = version.offset();
        let adjusted = |raw: u64| (raw as i64 + off) as u64;

        let mut missions = MemoryWatcherMap::new();
        for &(key, _, address) in MISSIONS {
            missions.insert(key, adjusted(address));
        }

        let mut collectibles = MemoryWatcherMap::new();
        for &(key, _, address, _) in COLLECTIBLES {
            collectibles.insert(key, adjusted(address));
        }

        // These values have separate Japanese addresses entirely.
        let game_state = if version == Version::Japanese {
            0x50387C
        } else {
            adjusted(0x505A2C)
        };
        let progress_made = if version == Version::Japanese {
            0x50436C
        } else {
            adjusted(0x50651C)
        };
        let mission_text = if version == Version::Japanese {
            0x272AE8
        } else {
            adjusted(0x274F20)
        };

        Self {
            missions,
            collectibles,
            game_state: MemoryWatcher::new(game_state),
            mission_text: MemoryWatcher::new(mission_text),
            progress_made: MemoryWatcher::new(progress_made),
            te_helipad: MemoryWatcher::new(adjusted(0x35F6B8)),
            te_timer: MemoryWatcher::new(adjusted(0x35BA2C)),
        }
    }

    pub fn update(&mut self, process: &Process, base: Address) {
        self.missions.update_all(process, base);
        self.collectibles.update_all(process, base);
        self.game_state.update(process, base);
        self.progress_made.update(process, base);
        self.mission_text.update(process, base);
        self.te_helipad.update(process, base);
        self.te_timer.update(process, base);
    }
}
