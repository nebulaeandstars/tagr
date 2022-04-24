use std::collections::HashSet;
use std::convert::Infallible;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use crate::crash;
use crate::file::File;

pub struct Tag {
    pub name: String,
}

impl Tag {
    pub fn get_members(&self) -> HashSet<File> {
        let tagfile = self.tagfile();
        if tagfile.exists() {
            let tagfile = fs::File::open(tagfile).unwrap();
            let reader = BufReader::new(tagfile);
            let lines = reader.lines();

            lines
                .into_iter()
                .map(|line| File::from_str(&line.unwrap()).unwrap())
                .collect()
        }
        else {
            crash!("no such tag: '{}'", self.name)
        }
    }

    /// Return the path to a tag's tagfile.
    ///
    /// For example:
    /// ```rust
    /// let tag = Tag::from("example");
    ///
    /// let mut target_tagfile = dirs::data_dir().unwrap();
    /// target_tagfile.push("tagr");
    /// target_tagfile.push(&tag.name);
    /// target_tagfile.set_extension("tag");
    ///
    /// assert_eq!(tag.tagfile(), target_tagfile);
    /// ```
    pub fn tagfile(&self) -> PathBuf {
        let mut path = dirs::data_dir().unwrap();
        path.push("tagr");
        path.push(&self.name);
        path.set_extension("tag");
        path
    }
}

impl FromStr for Tag {
    type Err = Infallible;
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(name))
    }
}

impl<S: AsRef<str>> From<S> for Tag {
    fn from(s: S) -> Self {
        Self { name: s.as_ref().to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tagfile_uses_data_dir() {
        let tag = Tag::from("example");

        let mut target_tagfile = dirs::data_dir().unwrap();
        target_tagfile.push("tagr");
        target_tagfile.push(&tag.name);
        target_tagfile.set_extension("tag");

        assert_eq!(tag.tagfile(), target_tagfile);
    }
}
