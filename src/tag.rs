use std::collections::HashSet;
use std::convert::Infallible;
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
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
            HashSet::new()
        }
    }

    pub fn add(&self, file: File) {
        let mut files = self.get_members();
        files.insert(file);

        let out: String = files
            .iter()
            .map(|file| file.full_path())
            .fold(String::new(), |out, path| {
                out + path.to_str().unwrap() + "\n"
            });

        let mut tagfile = fs::File::create(self.tagfile()).unwrap();
        tagfile.write_all(out.as_bytes()).unwrap();
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
