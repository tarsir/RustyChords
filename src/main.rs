mod session_config;
mod chord_types;
mod theory;

use std::io;
use crate::session_config::config;
use crate::chord_types::ChordTypes;
use crate::theory::notes::{get_note_index, note_as_flat};
use crate::theory::keys;

fn main() {
    let mut session_config: config::SessionConfig = config::default_config();
    use crate::theory::NOTES as notes;

    for n in notes.iter() {
        keys::get_key_signature(n.to_string(), notes);
    }

    // println!("Rusty Chord Finder");
    // println!("Enter notes in the chromatic scale, type 'config' to change the chord, or type `quit` to exit");

    // input_loop(notes, &mut session_config);

    // println!("Thanks for using Rusty Chords!");
}

fn input_loop(notes: [&str; 12], mut s_config: &mut config::SessionConfig) -> bool {
    loop {
        print!("> ");

        let mut user_note = String::new();

        io::stdin()
            .read_line(&mut user_note)
            .expect("Could not read input");

        if user_note.trim() == "quit" { // need to trim to get rid of the `read_line` newline
            break
        }

        if user_note.trim() == "config" {
            s_config = config_menu(s_config);
            config::print_config(s_config);
            continue
        }

        let user_note_index: usize;
        match get_note_index(&user_note, notes) {
            None => {
                println!("That is not a chromatic note!");
                continue;
            }
            Some(i) => user_note_index = i,
        };

        println!("Your note is: {}", user_note);
        println!("The index of this note is: {}", user_note_index);

        println!("This chord is: {}", get_chord(&user_note, notes, &s_config));
    }
    true
}

fn config_menu(s_config: &mut config::SessionConfig) -> &mut config::SessionConfig {
    config::print_config(s_config);

    println!("What do you want to set the generator to?");
    println!("Available: major, minor, min7, Maj7, dim, aug");
    println!("You can also enter 'sharp' or 'flat' to change the accidental display");

    let mut user_setting = String::new();

    io::stdin()
        .read_line(&mut user_setting)
        .expect("Could not read input");

    user_setting.make_ascii_lowercase();
    user_setting = user_setting.trim().to_string();

    // TODO: change the 7th detection to be independent of the full input (eg not this match)
    match user_setting.trim_end_matches("7") {
        "major" | "maj" => s_config.chord_type = ChordTypes::Major,
        "minor" | "min" => s_config.chord_type = ChordTypes::Minor,
        "dim" => s_config.chord_type = ChordTypes::Diminished,
        "aug" => s_config.chord_type = ChordTypes::Augmented,
        _ => ()
    };

    match user_setting.as_str() {
        "sharp" => s_config.show_flats = false,
        "flat" => s_config.show_flats = true,
        _ => ()
    };

    if user_setting.ends_with("7") {
        s_config.add_seven = true;
    }

    return s_config;
}

fn get_chord(note: &String, notes: [&str; 12], s_config: &config::SessionConfig) -> String {
    let root_index: usize = get_note_index(note, notes).unwrap();
    let third_offset: usize = match s_config.chord_type {
        ChordTypes::Major | ChordTypes::Augmented => 4,
        ChordTypes::Minor | ChordTypes::Diminished => 3,
    };
    let third_index: usize = (root_index + third_offset) % 12;
    let fifth_offset: usize = match s_config.chord_type {
        ChordTypes::Diminished => 6,
        ChordTypes::Major | ChordTypes::Minor => 7,
        ChordTypes::Augmented => 8
    };
    let fifth_index: usize = (root_index + fifth_offset) % 12;

    let mut chord_notes: Vec<String> = if s_config.add_seven {
        let seventh_index: usize = (root_index + 10) % 12;
        [note.trim(), notes[third_index], notes[fifth_index], notes[seventh_index]]
            .iter()
            .map(|n| n.to_string())
            .collect()
    } else {
        [note.trim(), notes[third_index], notes[fifth_index]]
            .iter()
            .map(|n| n.to_string())
            .collect()
    };

    chord_notes = if s_config.show_flats {
        chord_notes.iter().map(|note| note_as_flat(note, notes)).collect()
    } else {
        chord_notes
    };

    // return join_strings(" - ".to_string(), chord_notes);
    return chord_notes.join(" - ");
}
