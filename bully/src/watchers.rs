use alloc::{
    collections::btree_map::BTreeMap,
    string::{String, ToString},
};
use asr::{watcher::Watcher, Address, Process};

use crate::{
    helpers::{get_errand_key, get_gnome_key},
    missions::{ERRANDS_COUNT, GNOMES_COUNT, MISSIONS},
};

const MISSION_ARRAY_POINTER: u64 = 0x1CC4328;
const GNOME_BASE: u64 = 0x7CEB54;
const GNOME_STRIDE: u64 = 0x2;
const ERRANDS_BASE: u64 = 0x81B096;
const ERRANDS_STRIDE: u64 = 0x4;

pub struct Watchers {
    pub igt: Watcher<u32>,
    pub m1_state: Watcher<u8>,
    pub missions: BTreeMap<String, Watcher<u8>>,
    pub gnomes: BTreeMap<String, Watcher<u8>>,
    pub errands: BTreeMap<String, Watcher<u8>>,
}

impl Default for Watchers {
    fn default() -> Self {
        Self::new()
    }
}

impl Watchers {
    pub fn new() -> Self {
        let mut missions_map: BTreeMap<String, Watcher<u8>> = BTreeMap::new();
        let mut gnomes_map: BTreeMap<String, Watcher<u8>> = BTreeMap::new();
        let mut errands_map: BTreeMap<String, Watcher<u8>> = BTreeMap::new();

        for &(_, _, missions) in MISSIONS {
            for &(key, _, _, _) in missions {
                missions_map.insert(key.to_string(), Watcher::new());
            }
        }

        for i in 0..GNOMES_COUNT {
            gnomes_map.insert(get_gnome_key(i), Watcher::new());
        }

        for i in 0..ERRANDS_COUNT {
            errands_map.insert(get_errand_key(i), Watcher::new());
        }

        Self {
            igt: Watcher::new(),
            m1_state: Watcher::new(),
            missions: missions_map,
            gnomes: gnomes_map,
            errands: errands_map,
        }
    }

    pub fn update(&mut self, process: &Process, base: Address) {
        self.igt.update(
            process
                .read_pointer_path(base, asr::PointerSize::Bit32, &[0x81A340])
                .ok(),
        );

        self.m1_state.update(
            process
                .read_pointer_path(
                    base,
                    asr::PointerSize::Bit32,
                    &[MISSION_ARRAY_POINTER, 0x1C],
                )
                .ok(),
        );

        for &(_, _, missions) in MISSIONS {
            for &(key, _, pointer, _) in missions {
                if let Some(watcher) = self.missions.get_mut(key) {
                    watcher.update(
                        process
                            .read_pointer_path(
                                base,
                                asr::PointerSize::Bit32,
                                &[MISSION_ARRAY_POINTER, pointer.into()],
                            )
                            .ok(),
                    );
                }
            }
        }

        for i in 0..GNOMES_COUNT {
            let key = &get_gnome_key(i);
            let offset = GNOME_BASE + GNOME_STRIDE * i;

            if let Some(watcher) = self.gnomes.get_mut(key) {
                watcher.update(
                    process
                        .read_pointer_path(base, asr::PointerSize::Bit32, &[offset])
                        .ok(),
                );
            }
        }

        for i in 0..ERRANDS_COUNT {
            let key = &get_errand_key(i);
            let offset = ERRANDS_BASE + ERRANDS_STRIDE * i;

            if let Some(watcher) = self.errands.get_mut(key) {
                watcher.update(
                    process
                        .read_pointer_path(base, asr::PointerSize::Bit32, &[offset])
                        .ok(),
                );
            }
        }
    }
}
