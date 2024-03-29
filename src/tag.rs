use std::collections::HashSet;
use std::convert::Infallible;
use std::fs;
use std::hash::Hash;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::str::FromStr;

use crate::crash;
use crate::file::File;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tag {
    pub name: String,
}

impl Tag {
    pub fn add(&self, files: Vec<File>) {
        let mut tagged_files = self.get_members();

        for file in files {
            if tagged_files.contains(&file) {
                eprintln!(
                    "couldn't add '{}' to '{}': already tagged!",
                    file, self.name
                );
            }
            else {
                tagged_files.insert(file.clone());
                println!("added '{}' to '{}'", file, self.name);
            }
        }

        self.write_to_tagfile(tagged_files)
    }

    pub fn remove(&self, files: Vec<File>) {
        let mut tagged_files = self.get_members();
        if tagged_files.is_empty() {
            crash!("no such tag: '{}'", self.name)
        }

        for file in files {
            let removed = tagged_files.remove(&file);
            if removed {
                println!("removed '{}' from '{}'", file, self.name);
            }
            else {
                eprintln!(
                    "couldn't remove {} from {}: not in tag!",
                    file, self.name
                );
            }
        }

        if tagged_files.is_empty() {
            fs::remove_file(self.tagfile()).unwrap();
        }
        else {
            self.write_to_tagfile(tagged_files);
        }
    }

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

    pub fn list_members(&self) {
        let members = self.get_members();

        if members.is_empty() {
            crash!("no such tag: '{}'", self.name)
        }
        else {
            for file in self.get_members() {
                println!("{}", file);
            }
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

    fn write_to_tagfile(&self, files: HashSet<File>) {
        let out: String = files
            .iter()
            .map(|file| file.full_path())
            .fold(String::new(), |out, path| {
                out + path.to_str().unwrap() + "\n"
            });

        let tagfile = self.tagfile();
        let parent = tagfile.parent().unwrap();
        std::fs::create_dir_all(parent).unwrap();

        let mut tagfile = fs::File::create(tagfile).unwrap();
        tagfile.write_all(out.as_bytes()).unwrap();
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
