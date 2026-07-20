use asr::{string::ArrayCString, Address, Process};
use autosplitter_helpers::{MemoryWatcher, MemoryWatcherMap};

use crate::version::Version;

pub struct Watchers {
    pub start_flag: MemoryWatcher<u32>,
    pub progress_percent: MemoryWatcher<u32>,
    pub cutscene_load: MemoryWatcher<u32>,
    pub cutscene: MemoryWatcher<ArrayCString<255>>,
    pub save_load: MemoryWatcher<u8>,
    pub counters: MemoryWatcherMap<u32>,
    last_cutscene: ArrayCString<255>,
}

impl Watchers {
    pub fn new(version: Version) -> Self {
        let start_flag = if version == Version::GOG {
            0x1F870A0
        } else {
            0x1F870C0
        };

        let mut counters = MemoryWatcherMap::new();
        counters.insert("missions", 0x1053384);
        counters.insert("strongholds", 0x10533C8);
        counters.insert("tags", 0x10535E8);
        counters.insert("cd", 0x27C7150);
        counters.insert("jumps", 0x10535A4);
        counters.insert("barnstorming", 0x1053670);
        counters.insert("chop_shop", 0x10536B4);
        counters.insert("crowd_control", 0x10537C4);
        counters.insert("derby", 0x1053890);
        counters.insert("escort", 0x10539A0);
        counters.insert("fight_club", 0x1053A28);
        counters.insert("fuzz", 0x1053AB0);
        counters.insert("heli_assault", 0x1053B38);
        counters.insert("hitman", 0x10536F8);
        counters.insert("fraud", 0x1053D14);
        counters.insert("mayhem", 0x1053E68);
        counters.insert("races", 0x1055760);
        counters.insert("septic", 0x1053F34);
        counters.insert("snatch", 0x1054000);
        counters.insert("trafficking", 0x1053918);
        counters.insert("trail_blazing", 0x1053C04);

        Self {
            start_flag: MemoryWatcher::new(start_flag),
            progress_percent: MemoryWatcher::new(0x1052C58),
            cutscene_load: MemoryWatcher::new(0xA9D670),
            cutscene: MemoryWatcher::new([0x02127D10, 0x4, 0x0]),
            save_load: MemoryWatcher::new(0xA8EB88),
            counters,
            last_cutscene: ArrayCString::new(),
        }
    }

    pub fn update(&mut self, process: &Process, base: Address) {
        self.start_flag.update(process, base);
        self.progress_percent.update(process, base);
        self.cutscene_load.update(process, base);
        self.cutscene.update(process, base);
        if let Some(cutscene) = self.cutscene.pair() {
            if cutscene.current.validate_utf8().is_ok() {
                self.last_cutscene = cutscene.current;
            }
        }
        self.save_load.update(process, base);
        self.counters.update_all(process, base);
    }

    pub fn current_cutscene(&self) -> &str {
        self.last_cutscene.validate_utf8().unwrap_or("")
    }
}
