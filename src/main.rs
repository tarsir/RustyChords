mod session_config;
use std::io;
use crate::session_config::config;

fn main() {
    let notes: [&str; 12] = [
        "A",
        "A#",
        "B",
        "C",
        "C#",
        "D",
        "D#",
        "E",
        "F",
        "F#",
        "G",
        "G#"
    ];

    let mut session_config: config::SessionConfig = config::default_config();

    println!("Rusty Chord Finder");
    println!("Enter notes in the chromatic scale, or type `quit` to exit");

    input_loop(notes, &mut session_config);

    println!("Thanks for using Rusty Chords!");
}

fn input_loop(notes: [&str; 12], s_config: &mut config::SessionConfig) -> bool {
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
            *s_config = config_menu(&s_config);
            config::print_config(&s_config);
            continue
        }

        let user_note_index = get_note_index(&user_note, notes).unwrap();

        println!("Your note is: {}", user_note);
        println!("The index of this note is: {}", user_note_index);

        println!("This chord is: {}", get_chord(&user_note, notes, &s_config));
    }
    return true
}

fn config_menu(s_config: &config::SessionConfig) -> config::SessionConfig {
    config::print_config(s_config);

    println!("What do you want to set the generator to?");
    println!("Available: major, minor, min7, Maj7");

    let mut user_setting = String::new();

    io::stdin()
        .read_line(&mut user_setting)
        .expect("Could not read input");

    let s_config = match user_setting.trim() {
        "major" => config::SessionConfig{ is_major: true, add_seven: false },
        "minor" => config::SessionConfig{ is_major: false, add_seven: false },
        "min7" => config::SessionConfig{ is_major: false, add_seven: true },
        "Maj7" => config::SessionConfig{ is_major: true, add_seven: true },
        _ => config::default_config()
    };

    return s_config;
}

fn get_note_index(note: &String, notes: [&str; 12]) -> Result<usize, &'static str> {
    let mut index_ptr: usize = 0;
    let mut found: bool = false;

    for n in notes.iter() {
        match *n == note.trim() {
            true => found = true,
            _ => (index_ptr += 1)
        }

        if found {
            break;
        }
    }
    if found {
        return Ok(index_ptr)
    } else {
        return Err("The input does not match a note")
    }
}

fn get_chord(note: &String, notes: [&str; 12], s_config: &config::SessionConfig) -> String {
    let root_index: usize = get_note_index(note, notes).unwrap();
    let third_offset: usize = if s_config.is_major { 4 } else { 3 };
    let third_index: usize = (root_index + third_offset) % 12;
    let fifth_index: usize = (root_index + 7) % 12;
    return format!("{} - {} - {}", note.trim(), notes[third_index], notes[fifth_index]);
}