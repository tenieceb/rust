pub mod journal;
use crate::journal::entry::EmotionEntry;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use chrono::Local;
use chrono::NaiveDate;

pub fn display_entry(entry: &EmotionEntry) {
    println!("===============================");
    println!("Date:    {}", entry.date);
    println!("Emotion: {}", entry.emotion);
    println!("Color:   {}", entry.color);
    println!("Note:    {}", entry.note.as_deref().unwrap_or("None"));
    println!("===============================");
}

pub fn display_entries(entries: &[EmotionEntry]) {
    for entry in entries {
        display_entry(entry);
        println!();
    }
}

fn escape_csv_field(s: &str) -> String {
    if s.contains(',') || s.contains('"') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn unescape_csv_field(s: &str) -> String {
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        let inner = &s[1..s.len() - 1];
        inner.replace("\"\"", "\"")
    } else {
        s.to_string()
    }
}

pub fn save_all_entries(filename: &str, entries: &[EmotionEntry]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for entry in entries {
        writeln!(
            file,
            "{},{},{},{}",
            entry.date,
            entry.emotion,
            entry.color,
            escape_csv_field(entry.note.as_deref().unwrap_or(""))
        )?;
    }
    Ok(())
}

pub fn save_or_update_today_entry(filename: &str, new_entry: EmotionEntry) -> io::Result<()> {
    let mut entries = load_entries(filename).unwrap_or_else(|_| Vec::new());
    let today = Local::now().format("%Y-%m-%d").to_string();

    if let Some(pos) = entries.iter().position(|e| e.date == today) {
        entries[pos] = new_entry;
    } else {
        entries.push(new_entry);
    }

    save_all_entries(filename, &entries)
}

pub fn load_entries(filename: &str) -> io::Result<Vec<EmotionEntry>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.splitn(4, ',').collect();
        if parts.len() >= 3 {
            let note = if parts.len() == 4 && !parts[3].is_empty() {
                Some(unescape_csv_field(parts[3]))
            } else {
                None
            };

            entries.push(EmotionEntry {
                date: parts[0].to_string(),
                emotion: parts[1].to_string(),
                color: parts[2].to_string(),
                note,
            });
        }
    }
     entries.sort_by_key(|e| NaiveDate::parse_from_str(&e.date, "%Y-%m-%d").unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()));

    Ok(entries)
}
