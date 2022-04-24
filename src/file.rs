use std::convert::Infallible;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fmt, hash};

use crate::crash;

pub struct File {
    path: PathBuf,
}

impl File {
    pub fn validate_or_crash(&self) {
        if !self.path.exists() {
            crash!("file not found: {}", self)
        }
        else if !self.path.is_file() {
            crash!("path is not a file: {}", self)
        }
    }

    pub fn relative_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn full_path(&self) -> PathBuf {
        self.path.canonicalize().unwrap()
    }
}

impl FromStr for File {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from_str(s)?;
        let file = Self { path };
        file.validate_or_crash();
        Ok(file)
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = &self.relative_path().to_string_lossy();
        write!(f, "{}", path)
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let full_path = self.full_path();
        let path = full_path.to_string_lossy();
        write!(f, "{}", path)
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.full_path() == other.full_path()
    }
}
impl Eq for File {}

impl hash::Hash for File {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.full_path().hash(state)
    }
}
