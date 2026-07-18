use asr::{string::ArrayCString, watcher::Watcher, Address, Process};

use crate::version::Version;

pub struct Watchers {
    pub start_flag: Watcher<u32>,
    pub progress_percent: Watcher<u32>,
    pub cutscene_load: Watcher<u32>,
    pub cutscene: Watcher<ArrayCString<255>>,
    pub save_load: Watcher<u8>,
    pub missions: Watcher<u32>,
    pub strongholds: Watcher<u32>,
    pub tags: Watcher<u32>,
    pub cd: Watcher<u32>,
    pub jumps: Watcher<u32>,
    pub barnstorming: Watcher<u32>,
    pub chop_shop: Watcher<u32>,
    pub crowd_control: Watcher<u32>,
    pub derby: Watcher<u32>,
    pub escort: Watcher<u32>,
    pub fight_club: Watcher<u32>,
    pub fuzz: Watcher<u32>,
    pub heli_assault: Watcher<u32>,
    pub hitman: Watcher<u32>,
    pub fraud: Watcher<u32>,
    pub mayhem: Watcher<u32>,
    pub races: Watcher<u32>,
    pub septic: Watcher<u32>,
    pub snatch: Watcher<u32>,
    pub trafficking: Watcher<u32>,
    pub trail_blazing: Watcher<u32>,
}

impl Default for Watchers {
    fn default() -> Self {
        Self::new()
    }
}

impl Watchers {
    pub fn new() -> Self {
        Self {
            start_flag: Watcher::new(),
            progress_percent: Watcher::new(),
            cutscene_load: Watcher::new(),
            cutscene: Watcher::new(),
            save_load: Watcher::new(),
            missions: Watcher::new(),
            strongholds: Watcher::new(),
            tags: Watcher::new(),
            cd: Watcher::new(),
            jumps: Watcher::new(),
            barnstorming: Watcher::new(),
            chop_shop: Watcher::new(),
            crowd_control: Watcher::new(),
            derby: Watcher::new(),
            escort: Watcher::new(),
            fight_club: Watcher::new(),
            fuzz: Watcher::new(),
            heli_assault: Watcher::new(),
            hitman: Watcher::new(),
            fraud: Watcher::new(),
            mayhem: Watcher::new(),
            races: Watcher::new(),
            septic: Watcher::new(),
            snatch: Watcher::new(),
            trafficking: Watcher::new(),
            trail_blazing: Watcher::new(),
        }
    }

    pub fn update(&mut self, process: &Process, base: Address, version: Version) {
        let base_address = base.value();
        let start_flag_addr: u64 = if version == Version::GOG {
            0x1F870A0
        } else {
            0x1F870C0
        };

        self.start_flag
            .update(process.read(base_address + start_flag_addr).ok());
        self.progress_percent
            .update(process.read(base_address + 0x1052C58).ok());
        self.cutscene_load
            .update(process.read(base_address + 0xA9D670).ok());
        self.cutscene.update(
            process
                .read_pointer_path(base, asr::PointerSize::Bit32, &[0x02127D10, 0x4, 0x0])
                .ok(),
        );
        self.save_load
            .update(process.read(base_address + 0xA8EB88).ok());

        self.missions
            .update(process.read(base_address + 0x1053384).ok());
        self.strongholds
            .update(process.read(base_address + 0x10533C8).ok());
        self.tags
            .update(process.read(base_address + 0x10535E8).ok());
        self.cd.update(process.read(base_address + 0x27C7150).ok());
        self.jumps
            .update(process.read(base_address + 0x10535A4).ok());
        self.barnstorming
            .update(process.read(base_address + 0x1053670).ok());
        self.chop_shop
            .update(process.read(base_address + 0x10536B4).ok());
        self.crowd_control
            .update(process.read(base_address + 0x10537C4).ok());
        self.derby
            .update(process.read(base_address + 0x1053890).ok());
        self.escort
            .update(process.read(base_address + 0x10539A0).ok());
        self.fight_club
            .update(process.read(base_address + 0x1053A28).ok());
        self.fuzz
            .update(process.read(base_address + 0x1053AB0).ok());
        self.heli_assault
            .update(process.read(base_address + 0x1053B38).ok());
        self.hitman
            .update(process.read(base_address + 0x10536F8).ok());
        self.fraud
            .update(process.read(base_address + 0x1053D14).ok());
        self.mayhem
            .update(process.read(base_address + 0x1053E68).ok());
        self.races
            .update(process.read(base_address + 0x1055760).ok());
        self.septic
            .update(process.read(base_address + 0x1053F34).ok());
        self.snatch
            .update(process.read(base_address + 0x1054000).ok());
        self.trafficking
            .update(process.read(base_address + 0x1053918).ok());
        self.trail_blazing
            .update(process.read(base_address + 0x1053C04).ok());
    }
}
