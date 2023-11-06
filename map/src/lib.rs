use std::path::Path;

pub mod object;

pub mod parser {
    pub mod crlf;
    pub mod profile;
    pub mod slk;
}

pub struct Map {}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

impl Map {
    pub fn new() -> Self {
        Self {}
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        Self {}
    }
}
