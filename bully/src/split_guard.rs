use alloc::{
    collections::btree_map::BTreeMap,
    format,
    string::{String, ToString},
};

use crate::{
    helpers::get_gnome_key,
    missions::{GNOMES_COUNT, MISSIONS},
    watchers::Watchers,
};

pub struct SplitGuard {
    pub missions_done: BTreeMap<String, bool>,
    pub gnomes_done: BTreeMap<String, bool>,
}

impl Default for SplitGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl SplitGuard {
    pub fn new() -> Self {
        Self {
            missions_done: BTreeMap::new(),
            gnomes_done: BTreeMap::new(),
        }
    }

    pub fn clear(&mut self) {
        asr::print_message("Clearing split guard list");
        self.missions_done = BTreeMap::new();
    }

    pub fn refresh_finished_missions_list(&mut self, watchers: &Watchers) {
        for &(_, _, missions) in MISSIONS {
            for &(key, _, _, _) in missions {
                if let Some(watcher) = watchers.missions.get(key) {
                    if let Some(pair) = watcher.pair() {
                        if pair.current > 0 {
                            asr::print_message(&format!(
                                "Marking mission {key} as finished in split guard"
                            ));
                            self.missions_done.insert(key.to_string(), true);
                        }
                    }
                }
            }
        }

        let mut collected_gnome_count: u8 = 0;
        for i in 0..GNOMES_COUNT {
            let key = get_gnome_key(i);
            if let Some(watcher) = watchers.gnomes.get(i as usize) {
                if let Some(pair) = watcher.pair() {
                    if pair.current > 0 {
                        collected_gnome_count = collected_gnome_count + 1;
                        asr::print_message(&format!(
                            "Marking gnome {key} as finished in split guard"
                        ));
                        self.gnomes_done.insert(key, true);
                    }
                }
            }
        }

        if collected_gnome_count == 25 {
            asr::print_message("Marking all gnomes collected as finished in split guard");
            self.gnomes_done.insert("all_gnomes".to_string(), true);
        }
    }
}
