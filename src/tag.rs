use std::path::PathBuf;
use std::str::FromStr;

pub struct Tag {
    pub name:    String,
    pub tagfile: PathBuf,
}

impl FromStr for Tag {
    type Err = String;
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let mut tagfile = dirs::data_dir().unwrap();
        tagfile.push("tagr");
        tagfile.push(name);
        tagfile.set_extension("tag");

        Ok(Self { name: name.to_string(), tagfile })
    }
}
