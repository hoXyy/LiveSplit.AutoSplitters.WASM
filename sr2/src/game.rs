use asr::{
    watcher::{Pair, Watcher},
    Address, Process,
};

pub struct GameProcess {
    pub process: Process,
    pub state: State,
}

impl GameProcess {
    pub fn connect(process_name: &str) -> Option<Self> {
        let process = Process::attach(process_name)?;
        let base_address = process.get_module_address("sr2_pc.exe").unwrap();

        Some(Self {
            process,
            state: State::setup(base_address),
        })
    }
}

pub struct Variable<T> {
    var: Watcher<T>,
    base_address: Address,
    address_path: Vec<u32>,
}

impl<T: bytemuck::Pod + std::fmt::Debug> Variable<T> {
    pub fn update(&mut self, process: &Process) -> Option<&Pair<T>> {
        self.var.update(
            process
                .read_pointer_path32(self.base_address.0.try_into().unwrap(), &self.address_path)
                .ok(),
        )
    }
}

pub struct State {
    pub cutscene: Variable<[u8; 255]>,
    pub start_flag: Variable<i32>,
    pub completion: Variable<i32>,
    pub cutscene_load: Variable<i32>,
    pub save_load: Variable<i8>,
    pub missions: Variable<i32>,
    pub strongholds: Variable<i32>,
    pub tags: Variable<i32>,
    pub cds: Variable<i32>,
    pub jumps: Variable<i32>,
    pub barnstorming: Variable<i32>,
    pub chop_shop: Variable<i32>,
    pub crowd_control: Variable<i32>,
    pub derby: Variable<i32>,
    pub escort: Variable<i32>,
    pub fight_club: Variable<i32>,
    pub fuzz: Variable<i32>,
    pub heli_assault: Variable<i32>,
    pub hitman: Variable<i32>,
    pub fraud: Variable<i32>,
    pub mayhem: Variable<i32>,
    pub races: Variable<i32>,
    pub septic: Variable<i32>,
    pub snatch: Variable<i32>,
    pub trafficking: Variable<i32>,
    pub trail_blazing: Variable<i32>,
}

impl State {
    fn setup(base_address: Address) -> Self {
        Self {
            cutscene: Variable { 
                var: Watcher::new(),
                base_address,
                address_path: vec![0x02127D10, 0x4, 0x0],
            },
            start_flag: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1F870A0],
            },
            completion: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1052C58],
            },
            cutscene_load: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0xA9D670],
            },
            save_load: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0xA8EB88],
            },
            missions: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1053384],
            },
            strongholds: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x10533C8],
            },
            tags: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x10535E8],
            },
            cds: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x27C7150],
            },
            jumps: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x10535A4],
            },
            barnstorming: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1053670],
            },
            chop_shop: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x10536B4],
            },
            crowd_control: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x10537C4],
            },
            derby: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1053890],
            },
            escort: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x10539A0],
            },
            fight_club: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1053A28],
            },
            fuzz: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1053AB0],
            },
            heli_assault: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1053B38],
            },
            hitman: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x10536F8],
            },
            fraud: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1053D14],
            },
            mayhem: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1053E68],
            },
            races: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1055760],
            },
            septic: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1053F34],
            },
            snatch: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1054000],
            },
            trafficking: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1053918],
            },
            trail_blazing: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1053C04],
            },
        }
    }
}

impl State {
    pub fn update(&mut self, process: &Process) -> Option<Variables> {
        Some(Variables {
            cutscene: self.cutscene.update(process),
            start_flag: self.start_flag.update(process)?,
            completion: self.completion.update(process)?,
            cutscene_load: self.cutscene_load.update(process)?,
            save_load: self.save_load.update(process)?,
            missions: self.missions.update(process)?,
            strongholds: self.strongholds.update(process)?,
            tags: self.tags.update(process)?,
            cds: self.cds.update(process)?,
            jumps: self.jumps.update(process)?,
            barnstorming: self.barnstorming.update(process)?,
            chop_shop: self.chop_shop.update(process)?,
            crowd_control: self.crowd_control.update(process)?,
            derby: self.derby.update(process)?,
            escort: self.escort.update(process)?,
            fight_club: self.fight_club.update(process)?,
            fuzz: self.fuzz.update(process)?,
            heli_assault: self.heli_assault.update(process)?,
            hitman: self.hitman.update(process)?,
            fraud: self.fraud.update(process)?,
            mayhem: self.mayhem.update(process)?,
            races: self.races.update(process)?,
            septic: self.septic.update(process)?,
            snatch: self.snatch.update(process)?,
            trafficking: self.trafficking.update(process)?,
            trail_blazing: self.trail_blazing.update(process)?,
        })
    }
}

pub struct Variables<'a> {
    pub cutscene: Option<&'a Pair<[u8; 255]>>,
    pub start_flag: &'a Pair<i32>,
    pub completion: &'a Pair<i32>,
    pub cutscene_load: &'a Pair<i32>,
    pub save_load: &'a Pair<i8>,
    pub missions: &'a Pair<i32>,
    pub strongholds: &'a Pair<i32>,
    pub tags: &'a Pair<i32>,
    pub cds: &'a Pair<i32>,
    pub jumps: &'a Pair<i32>,
    pub barnstorming: &'a Pair<i32>,
    pub chop_shop: &'a Pair<i32>,
    pub crowd_control: &'a Pair<i32>,
    pub derby: &'a Pair<i32>,
    pub escort: &'a Pair<i32>,
    pub fight_club: &'a Pair<i32>,
    pub fuzz: &'a Pair<i32>,
    pub heli_assault: &'a Pair<i32>,
    pub hitman: &'a Pair<i32>,
    pub fraud: &'a Pair<i32>,
    pub mayhem: &'a Pair<i32>,
    pub races: &'a Pair<i32>,
    pub septic: &'a Pair<i32>,
    pub snatch: &'a Pair<i32>,
    pub trafficking: &'a Pair<i32>,
    pub trail_blazing: &'a Pair<i32>,
}

impl<'a> Variables<'a> {
    pub fn get_as_string(var: &'a [u8]) -> Option<&'a str> {
        let null_pos = var.iter().position(|&x| x == b'\0').unwrap_or(var.len());

        std::str::from_utf8(&var[0..null_pos]).ok()
    }
}
