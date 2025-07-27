use std::io::{self};
mod storage;
mod journal;
mod display;
use chrono::Local;
use crate::journal::entry::EmotionEntry;





fn main() {
    println!("Hello, Welcome to MoodStitch: the Color Emotion Journal!");
    println!("This journal helps you track your emotions and associate them with colors.");
    println!("Over time, it creates a visual representation for your mood blanket!");
    println!("\nPlease select an option:");
    println!("1. Load existing journal");
    println!("2. Create a new journal");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    let mut journal_file = String::new();

    match choice {
        "1" => {
            println!("Great! Let's load your journal.");
            println!("Enter your journal file name (e.g., my_journal.txt):");
            io::stdin().read_line(&mut journal_file).unwrap();
        }
        "2" => {
            println!("Creating a new journal!");
            println!("Enter a name for your new journal file (e.g., my_journal.txt):");
            io::stdin().read_line(&mut journal_file).unwrap();
        }
        _ => {
            println!("Invalid option. Exiting.");
            return;
        }
    }

    let journal_file = journal_file.trim();
    println!("Using journal file: {}\n", journal_file);

    // Choose emotion
    println!("Select the emotion you want to log today:");
    println!("1. Happy\n2. Sad\n3. Angry\n4. Anxious\n5. Excited\n6. Relaxed\n7. Grateful\n8. Other");

    let mut emotion_choice = String::new();
    io::stdin().read_line(&mut emotion_choice).unwrap();
    let emotion_choice = emotion_choice.trim();

    let emotion = match emotion_choice {
        "1" => "Happy".to_string(),
        "2" => "Sad".to_string(),
        "3" => "Angry".to_string(),
        "4" => "Anxious".to_string(),
        "5" => "Excited".to_string(),
        "6" => "Relaxed".to_string(),
        "7" => "Grateful".to_string(),
        "8" => {
            println!("Please specify your emotion:");
            let mut other_emotion = String::new();
            io::stdin().read_line(&mut other_emotion).unwrap();
            other_emotion.trim().to_string()
        }
        _ => {
            println!("Invalid choice.");
            return;
        }
    };

    // Ask to add a note
    println!("Would you like to add a note? (yes/no)");
    let mut add_note = String::new();
    io::stdin().read_line(&mut add_note).unwrap();

    let note = if add_note.trim().eq_ignore_ascii_case("yes") {
        println!("Please enter your note:");
        let mut note_input = String::new();
        io::stdin().read_line(&mut note_input).unwrap();
        note_input.trim().to_string()
    } else {
        String::new()
    };

    // Show the entry
    println!("\nYour entry:");
    println!("Emotion: {}", emotion);
    if !note.is_empty() {
        println!("Note: {}", note);
    }

    // Create today's date string
    let today = Local::now().format("%Y-%m-%d").to_string();

    // Create the EmotionEntry struct
    let entry = EmotionEntry::new(today, emotion.clone(), if note.is_empty() { None } else { Some(note.clone()) });

    // Ask to save the entry
    println!("\nWould you like to save this entry? (yes/no)");
    let mut save_entry = String::new();
    io::stdin().read_line(&mut save_entry).unwrap();

    if save_entry.trim().eq_ignore_ascii_case("yes") {
        println!("Saving entry to file...");
        match storage::save_or_update_today_entry(journal_file, entry) {
            Ok(_) => println!("Entry saved!"),
            Err(e) => println!("Failed to save entry: {}", e),
        }
    } else {
        println!("Entry not saved.");
    }

println!("\nWould you like to view previous entries? (yes/no)");
let mut view_entries = String::new();
io::stdin().read_line(&mut view_entries).unwrap();

if view_entries.trim().eq_ignore_ascii_case("yes") {
    println!("How many previous entries would you like to see?");
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let num: usize = match num.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number.");
            return;
        }
    };

    // Load all entries from the journal file
    match storage::load_entries(journal_file) {
        Ok(entries) => {
            let total = entries.len();
            let start = if num > total { 0 } else { total - num };
            let slice = &entries[start..];

            println!("\nShowing last {} entries from {}:", slice.len(), journal_file);
            display::display_entries(slice); 
        }
        Err(e) => println!("Failed to load entries: {}", e),
    }



    println!("\nThank you for using MoodStitch!");
}}  
