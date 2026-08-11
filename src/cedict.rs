use std::fs::File;
use std::path::Path;
use cedict;
use std::io::Result;

pub type DictEntry = cedict::DictEntry<String>;

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Vec<DictEntry>> {
    match File::open(path) {
        Ok(file) => {
            Ok(cedict::parse_reader(file).collect())
        },
        Err(e) => {
            println!("Cannot read file");
            Err(e)
        }
    }
}
