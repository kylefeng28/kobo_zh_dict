use std::io;
use clap::{Command, arg};

mod kobo;
mod generic_dict;
mod cedict;

use crate::generic_dict::{generate_entries, EntrySettings};

// Need to output as a Japanese dictionary even though this is for Chinese,
// since Kobo has specially handling for kanji/hanzi characters but only if the locale is "ja"
const DEFAULT_OUTPUT: &str = "dicthtml-ja-en.zip";
const DEFAULT_CEDICT: &str = "cedict_1_0_ts_utf-8_mdbg.txt";

fn main() -> io::Result<()> {
    let matches = Command::new("kobo_zh_dict")
        .about("Kobo Chinese Dictionary Builder")
        .arg(arg!(--cedict <CEDICT> "Path to CC-CEDICT file")
            .default_value(DEFAULT_CEDICT)
        )
        .arg(arg!(--output <OUTPUT> "The output filepath to write the new dictionary to.")
            .default_value(DEFAULT_OUTPUT)
        )
        .arg(arg!(-c --character <CHARACTER_MODE> "Character mode")
            .value_parser(["traditional", "simplified"])
            .default_value("simplified")
        )
        .arg(arg!(-p --pronunciation <PRONUNCIATION_MODE> "Pronunciation mode")
            .value_parser(["pinyin", "zhuyin"])
            .default_value("pinyin")
        )
        .get_matches();

    // Output zip archive path.
    let cedict_filename = matches.get_one::<String>("cedict").unwrap();
    let output_filename = matches.get_one::<String>("output").unwrap();
    let character_mode = match matches.get_one::<String>("character").unwrap().as_str() {
        "traditional" => generic_dict::CharacterMode::Traditional,
        "simplified" => generic_dict::CharacterMode::Simplified,
        _ => unreachable!()
    };
    let pronunciation_mode = match matches.get_one::<String>("pronunciation").unwrap().as_str() {
        "pinyin" => generic_dict::PronunciationMode::Pinyin,
        "zhuyin" => generic_dict::PronunciationMode::Zhuyin,
        _ => unreachable!()
    };

    let cedict_entries = cedict::parse_file(cedict_filename)?;

    let entry_settings = EntrySettings {
        add_separators: true,
        character_mode: character_mode.clone(),
        pronunciation_mode: pronunciation_mode.clone(),
    };

    println!("Using CC-CEDICT file at {}", cedict_filename);
    println!("Will output Kobo dictionary to {}", output_filename);
    println!("Character mode: {:?}", character_mode);
    println!("Pronunciation mode: {:?}", pronunciation_mode);
    println!();

    println!("Generating Kobo dictionary...");
    let entries = generate_entries(&cedict_entries, &entry_settings);
    kobo::write_dictionary(&entries, std::path::Path::new(output_filename))?;
    println!("Wrote Kobo dictionary to {} with {} entries", output_filename, entries.len());

    Ok(())
}
