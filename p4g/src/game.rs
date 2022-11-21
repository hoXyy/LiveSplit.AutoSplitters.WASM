use asr::{watcher::{Pair, Watcher}, Process};

pub struct GameProcess {
    pub process: Process,
    pub state: State,
}

impl GameProcess {
    pub fn connect(process_name: &str) -> Option<Self> {
        let process = Process::attach(process_name)?;

        Some(Self {
            process,
            state: State::default(),
        })
    }
}

#[derive(Default)]
pub struct Variable<T> {
    var: Watcher<T>,
    base_address: u32,
    address_path: Vec<u32>,
}

impl<T: bytemuck::Pod + std::fmt::Debug> Variable<T> {
    pub fn update(&mut self, process: &Process) -> Option<&Pair<T>> {
        self.var.update(process
            .read_pointer_path32(self.base_address, &self.address_path)
            .ok(),
        )
    }
}

pub struct State {
    pub loading: Variable<i16>,
}

impl Default for State {
    fn default() -> Self {
        let base_address = 0x400000;
        Self {
            loading: Variable { var: Watcher::new(), base_address, address_path: vec![0x49DC372] }
        }
    }
}

impl State {
    pub fn update(&mut self, process: &Process) -> Option<Variables> {
        Some(Variables {
            loading: self.loading.update(process)?,
        })
    }
}

pub struct Variables<'a> {
    pub loading: &'a Pair<i16>,
}