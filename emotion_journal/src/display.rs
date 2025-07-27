use crate::journal::entry::EmotionEntry;

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
