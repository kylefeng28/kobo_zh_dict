use std::io;

mod kobo;
mod generic_dict;
mod cedict;

use crate::generic_dict::{generate_entries, EntrySettings};

fn main() -> io::Result<()> {
    let cedict_entries = cedict::parse_file("./cedict_1_0_ts_utf-8_mdbg.txt")?;

    // Need to output as a Japanese dictionary even though this is for Chinese,
    // since Kobo has specially handling for kanji/hanzi characters but only if the locale is "ja"
    let output_filename = "dicthtml-ja-en.zip";

    let entry_settings = EntrySettings { add_separators: true };

    let entries = generate_entries(&cedict_entries, entry_settings);


    println!("Writing Kobo dictionary to disk...");
    kobo::write_dictionary(&entries, std::path::Path::new(output_filename))?;

    Ok(())
}
