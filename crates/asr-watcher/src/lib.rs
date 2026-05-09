#![no_std]

const GRACE_PERIOD_TICKS: u32 = 90;

#[derive(Copy, Clone)]
pub struct Watcher<T: Copy + Default> {
    pub current: T,
    pub old: T,
    pub valid: bool,
    invalidated_ticks: Option<u32>,
}

impl<T: Copy + Default> Default for Watcher<T> {
    fn default() -> Self {
        Self {
            current: T::default(),
            old: T::default(),
            valid: false,
            invalidated_ticks: None,
        }
    }
}

impl<T: Copy + Default> Watcher<T> {
    pub fn update(&mut self, value: T) {
        self.old = if self.valid { self.current } else { value };
        self.current = value;
        self.valid = true;
        self.invalidated_ticks = None;
    }

    pub fn invalidate(&mut self) {
        if self.valid && self.invalidated_ticks.is_none() {
            self.invalidated_ticks = Some(0);
        } else if let Some(ref mut ticks) = self.invalidated_ticks {
            *ticks += 1;
            if *ticks > GRACE_PERIOD_TICKS {
                self.valid = false;
                self.invalidated_ticks = None;
                self.old = T::default();
                self.current = T::default();
            }
        }
    }

    pub fn update_from<E>(&mut self, result: Result<T, E>) {
        match result {
            Ok(v) => self.update(v),
            Err(_) => self.invalidate(),
        }
    }
}
