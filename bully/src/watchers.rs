use alloc::vec::Vec;
use asr::{Address, Process};
use autosplitter_helpers::{MemoryWatcher, MemoryWatcherMap};

use crate::missions::{ERRANDS_COUNT, GNOMES_COUNT, MISSIONS};

const MISSION_ARRAY_POINTER: u64 = 0x1CC4328;
const GNOME_BASE: u64 = 0x7CEB54;
const GNOME_STRIDE: u64 = 0x2;
const ERRANDS_BASE: u64 = 0x81B096;
const ERRANDS_STRIDE: u64 = 0x4;

pub struct Watchers {
    pub igt: MemoryWatcher<u32>,
    pub m1_state: MemoryWatcher<u8>,
    pub missions: MemoryWatcherMap<u8>,
    pub gnomes: Vec<MemoryWatcher<u8>>,
    pub errands: Vec<MemoryWatcher<u8>>,
}

impl Default for Watchers {
    fn default() -> Self {
        Self::new()
    }
}

impl Watchers {
    pub fn new() -> Self {
        let mut missions = MemoryWatcherMap::new();
        for &(_, _, mission_entries) in MISSIONS {
            for &(key, _, pointer, _) in mission_entries {
                missions.insert(key, [MISSION_ARRAY_POINTER, pointer.into()]);
            }
        }

        Self {
            igt: MemoryWatcher::new(0x81A340),
            m1_state: MemoryWatcher::new([MISSION_ARRAY_POINTER, 0x1C]),
            missions,
            gnomes: (0..GNOMES_COUNT)
                .map(|i| MemoryWatcher::new(GNOME_BASE + GNOME_STRIDE * i))
                .collect(),
            errands: (0..ERRANDS_COUNT)
                .map(|i| MemoryWatcher::new(ERRANDS_BASE + ERRANDS_STRIDE * i))
                .collect(),
        }
    }

    pub fn update(&mut self, process: &Process, base: Address) {
        self.igt.update(process, base);
        self.m1_state.update(process, base);
        self.missions.update_all(process, base);
        for watcher in &mut self.gnomes {
            watcher.update(process, base);
        }
        for watcher in &mut self.errands {
            watcher.update(process, base);
        }
    }
}
