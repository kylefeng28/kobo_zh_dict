use once_cell::sync::Lazy;
use regex::Regex;
use pinyin_zhuyin::{encode_pinyin as encode_pinyin_, encode_zhuyin as encode_zhuyin_};
use std::fs;

use crate::cedict;

static SPLIT_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"[,\s·]+").unwrap());

pub struct Entry {
    // The integer here is a very rough priority ranking indicating
    // the commonness of the word, specifically in that form.  A
    // lower numerical value indicates a more common word.
    pub keys: Vec<(String, u32)>,
    pub definition: String,
}

#[derive(Debug, Clone)]
pub enum CharacterMode {
    Traditional,
    Simplified,
}

#[derive(Debug, Clone)]
pub enum PronunciationMode {
    Pinyin,
    Zhuyin,
}

#[derive(Debug, Clone)]
pub struct EntrySettings {
    /// Will add a horizontal bar at the top of each entry.  This is mainly for
    /// Kobo, which displays all entries together in a continuous page.
    pub add_separators: bool,
    pub character_mode: CharacterMode,
    pub pronunciation_mode: PronunciationMode,
}

pub fn generate_entries(
    cedict_entries: &Vec<cedict::DictEntry>,
    entry_settings: &EntrySettings,
) -> Vec<Entry> {
    let mut entries = Vec::new();
    let mut errors: Vec<String> = vec![];

    for entry in cedict_entries.iter() {
        entries.push(Entry {
            keys: generate_keys(&entry),
            definition: generate_definition_text(&entry, &entry_settings, &mut errors),
        });
    }

    let errors_file = "errors.txt";
    eprintln!("Found {} errors, saved to {}", errors.len(), errors_file);
    fs::write(errors_file, errors.join("\n")).unwrap();

    entries.sort_by_key(|a| a.keys[0].0.len());

    entries
}

fn generate_keys(entry: &cedict::DictEntry) -> Vec<(String, u32)> {
    let mut keys = vec![(entry.simplified().to_string(), 0)];
    if entry.simplified() != entry.traditional() {
        keys.push((entry.traditional().to_string(), 0));
    }
    keys
}

fn get_character_variants<'a>(
    entry: &'a cedict::DictEntry,
    entry_settings: &EntrySettings,
) -> (&'a str, &'a str) {
    match entry_settings.character_mode {
        CharacterMode::Traditional => (entry.traditional(), entry.simplified()),
        CharacterMode::Simplified => (entry.simplified(), entry.traditional()),
    }
}

fn normalize(s: &str) -> String {
    // Convert to lowercase and convert "u:" to "v" (representing ü)
    s.to_lowercase().replace("u:", "v")
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn encode_pinyin(s: &str) -> Option<String> {
    // Preserve capital letter from raw pinyin, e.g. Bei3 -> Běi, Ai4 -> Ài
    match s.chars().next() {
        Some(c) => {
            let encoded = encode_pinyin_(normalize(s));
            if c.is_uppercase() { encoded.map(|enc| capitalize_first(&enc)) }
            else { encoded }
        }
        None => None
    }
}

fn encode_zhuyin(s: &str) -> Option<String> {
    encode_zhuyin_(normalize(s))
}

fn get_pronunciation(
    entry: &cedict::DictEntry,
    entry_settings: &EntrySettings,
    errors: &mut Vec<String>,
) -> String {
    // CC-CEDICT entry is raw pinyin with numbers
    let raw_pinyin = entry.pinyin();
    let encode = match entry_settings.pronunciation_mode {
        PronunciationMode::Pinyin => encode_pinyin,
        PronunciationMode::Zhuyin => encode_zhuyin,
    };

    let result = SPLIT_PATTERN.split(raw_pinyin)
        .map(|syllable| {
            match encode(syllable) {
                Some(encoded) => encoded,
                None => {
                    errors.push(format!(
                            "{}: invalid pinyin syllable '{}' in '{}'",
                            entry.traditional(), syllable, raw_pinyin));

                    syllable.to_string()
                }
            }
        })
        .collect::<String>();

    result
}

fn generate_definition_text(
    entry: &cedict::DictEntry,
    entry_settings: &EntrySettings,
    errors: &mut Vec<String>,
) -> String {
    let mut text = String::new();
    if entry_settings.add_separators {
        text.push_str("<hr/>");
    }

    // Header
    // Need to force the default Kobo Chinese font, otherwise simplified or Chinese-only characters will not be displayed at all
    let font_style = "style=\"font-family: 'AR UDJingxihei' !important;\"";
    text.push_str(format!("<div {}>", font_style).as_str());
    let (preferred, alt) = get_character_variants(entry, entry_settings);
    text.push_str("<br>");
    text.push_str(preferred);
    if preferred != alt {
        text.push_str(" 【");
        text.push_str(alt);
        text.push_str("】");
    }
    text.push_str("</b> ");

    text.push_str(get_pronunciation(entry, entry_settings, errors).as_str());

    text.push_str("<br/>");

    for meaning in entry.definitions() {
        text.push_str("• ");
        text.push_str(meaning);
        text.push_str("<br/>");
    }
    text.pop();

    text.push_str("</div>");

    text
}
