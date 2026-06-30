use asr::{file_format::pe::read_size_of_image, Address, Process};

#[derive(Copy, Clone, PartialEq)]
pub enum Version {
    V10,
    V11,
    Steam,
    Japanese,
}

impl Version {
    pub fn offset(self) -> i64 {
        match self {
            Version::V10 | Version::V11 => -0x10140,
            Version::Steam => 0,
            Version::Japanese => -0x21E0,
        }
    }

    pub fn detect(process: &Process, base_address: Address) -> Option<Self> {
        let module_size = read_size_of_image(process, base_address).unwrap();
        match module_size {
            6197248 | 5836800 => return Some(Version::Steam),
            _ => {}
        }

        const VERSION_CHECK_NUMBER: u32 = 1407551829;
        let base = process.get_module_address("gta3.exe").ok()?;

        if process.read::<u32>(base + 0x1C1E70u64).ok()? == VERSION_CHECK_NUMBER {
            return Some(Version::V10);
        }
        if process.read::<u32>(base + 0x1C2130u64).ok()? == VERSION_CHECK_NUMBER {
            return Some(Version::V11);
        }
        if process.read::<u32>(base + 0x1B52D0u64).ok()? == VERSION_CHECK_NUMBER {
            return Some(Version::Japanese);
        }

        None
    }
}
