#![no_std]

extern crate alloc;

use alloc::{boxed::Box, collections::BTreeMap};
use asr::{
    watcher::{Pair, Watcher},
    Address, PointerSize, Process,
};
use bytemuck::CheckedBitPattern;
use core::ops::Index;

/// An owned pointer path relative to a module's base address.
pub struct MemoryPath(Box<[u64]>);

impl From<u64> for MemoryPath {
    fn from(offset: u64) -> Self {
        Self(Box::new([offset]))
    }
}

impl<const N: usize> From<[u64; N]> for MemoryPath {
    fn from(path: [u64; N]) -> Self {
        Self(Box::new(path))
    }
}

impl From<&[u64]> for MemoryPath {
    fn from(path: &[u64]) -> Self {
        Self(path.into())
    }
}

impl From<Box<[u64]>> for MemoryPath {
    fn from(path: Box<[u64]>) -> Self {
        Self(path)
    }
}

/// An ASR watcher paired with the pointer path used to update it.
pub struct MemoryWatcher<T> {
    watcher: Watcher<T>,
    path: MemoryPath,
}

impl<T> MemoryWatcher<T> {
    /// Creates a watcher for a module-relative offset or pointer path.
    pub fn new(path: impl Into<MemoryPath>) -> Self {
        Self {
            watcher: Watcher::new(),
            path: path.into(),
        }
    }

    /// Returns the watcher's current old/current value pair, if initialized.
    pub fn pair(&self) -> Option<&Pair<T>> {
        self.watcher.pair.as_ref()
    }
}

impl<T: CheckedBitPattern> MemoryWatcher<T> {
    /// Reads the value at this watcher's path from a 32-bit process.
    pub fn update(&mut self, process: &Process, base: Address) {
        self.watcher.update(
            process
                .read_pointer_path(base, PointerSize::Bit32, &self.path.0)
                .ok(),
        );
    }
}

macro_rules! impl_increased_by {
    ($($ty:ty),* $(,)?) => {
        $(
            impl MemoryWatcher<$ty> {
                /// Returns whether the value increased by exactly `amount`.
                pub fn increased_by(&self, amount: $ty) -> bool {
                    self.pair().is_some_and(|pair| {
                        pair.old
                            .checked_add(amount)
                            .is_some_and(|expected| pair.current == expected)
                    })
                }
            }
        )*
    };
}

impl_increased_by!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

/// A named collection of memory watchers of the same value type.
pub struct MemoryWatcherMap<T> {
    watchers: BTreeMap<&'static str, MemoryWatcher<T>>,
}

impl<T> MemoryWatcherMap<T> {
    /// Creates an empty watcher map.
    pub const fn new() -> Self {
        Self {
            watchers: BTreeMap::new(),
        }
    }

    /// Adds or replaces a named watcher.
    pub fn insert(&mut self, name: &'static str, path: impl Into<MemoryPath>) {
        self.watchers.insert(name, MemoryWatcher::new(path));
    }

    /// Returns the watcher registered under `name`.
    pub fn get(&self, name: &str) -> Option<&MemoryWatcher<T>> {
        self.watchers.get(name)
    }

    /// Iterates over the watchers in name order.
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &MemoryWatcher<T>)> {
        self.watchers.iter().map(|(&name, watcher)| (name, watcher))
    }
}

impl<T: CheckedBitPattern> MemoryWatcherMap<T> {
    /// Updates every watcher in the map.
    pub fn update_all(&mut self, process: &Process, base: Address) {
        for watcher in self.watchers.values_mut() {
            watcher.update(process, base);
        }
    }
}

impl<T> Default for MemoryWatcherMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<&str> for MemoryWatcherMap<T> {
    type Output = MemoryWatcher<T>;

    fn index(&self, name: &str) -> &Self::Output {
        self.get(name)
            .unwrap_or_else(|| panic!("memory watcher \"{name}\" was not registered"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserts_and_indexes_watchers_by_name() {
        let mut watchers = MemoryWatcherMap::<u32>::new();
        watchers.insert("missions", 0x1234);
        watchers.insert("collectibles", [0x5678, 0x10]);

        assert!(watchers.get("missions").is_some());
        assert!(watchers.get("missing").is_none());
        assert_eq!(
            watchers
                .iter()
                .map(|(name, _)| name)
                .collect::<alloc::vec::Vec<_>>(),
            ["collectibles", "missions"]
        );

        let _ = &watchers["missions"];
    }

    #[test]
    fn detects_exact_increases_without_overflowing() {
        let mut watcher = MemoryWatcher::<u8>::new(0);

        watcher.watcher.update_infallible(10);
        watcher.watcher.update_infallible(11);
        assert!(watcher.increased_by(1));
        assert!(!watcher.increased_by(2));

        watcher.watcher.update_infallible(u8::MAX);
        watcher.watcher.update_infallible(0);
        assert!(!watcher.increased_by(1));
    }
}
