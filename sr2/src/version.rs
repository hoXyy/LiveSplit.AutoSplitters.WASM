use asr::{file_format::pe::read_size_of_image, Address, Process};

#[derive(Copy, Clone, PartialEq)]
pub enum Version {
    GOG,
    Steam,
    SteamJuiced,
}

impl Version {
    pub fn detect(process: &Process, base_address: Address) -> Option<Self> {
        if let Some(module_size) = read_size_of_image(process, base_address) {
            if module_size == 0x31AD000 {
                return Some(Version::Steam);
            }

            if module_size == 0x31582FC {
                return Some(Version::SteamJuiced);
            }

            if module_size == 0x3159000 {
                return Some(Version::GOG);
            }

            None
        } else {
            None
        }
    }
}
