use crate::missions::{COLLECTIBLES, MISSIONS};

pub const MAX_COLLECTIBLE: usize = 100;

pub struct SplitGuard {
    pub missions_complete: [bool; MISSIONS.len()],
    pub missions_start: [bool; MISSIONS.len()],
    pub collectibles_all: [bool; COLLECTIBLES.len()],
    pub collectibles_each: [[bool; MAX_COLLECTIBLE]; COLLECTIBLES.len()],
}

impl Default for SplitGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl SplitGuard {
    pub fn new() -> Self {
        Self {
            missions_complete: [false; MISSIONS.len()],
            missions_start: [false; MISSIONS.len()],
            collectibles_all: [false; COLLECTIBLES.len()],
            collectibles_each: [[false; MAX_COLLECTIBLE]; COLLECTIBLES.len()],
        }
    }

    pub fn clear(&mut self) {
        self.missions_complete = [false; MISSIONS.len()];
        self.missions_start = [false; MISSIONS.len()];
        self.collectibles_all = [false; COLLECTIBLES.len()];
        self.collectibles_each = [[false; MAX_COLLECTIBLE]; COLLECTIBLES.len()];
    }
}
