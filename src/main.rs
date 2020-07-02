use std::io;

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

    println!("Enter a note");

    let mut user_note = String::new();

    io::stdin()
        .read_line(&mut user_note)
        .expect("Could not read input");

    println!("Your note is: {}", user_note);
    let user_note_index = get_note_index(user_note, notes);
    println!("The index of this note is: {}", user_note_index);

    println!("This major chord is: {}", get_major_chord(user_note, notes));
}

fn get_note_index(note: String, notes: [&str; 12]) -> usize {
    let mut index_ptr: usize = 0; 

    for n in notes.iter() {
        match *n == note.trim() {
            true => return index_ptr,
            _ => (index_ptr += 1)
        }
    }
    return -1;
}

fn get_major_chord(note: String, notes: [&str; 12]) -> String {
    let root_index = get_note_index(note, notes);
    let third_index: usize = (root_index + 4) % 12;
    let fifth_index: usize = (root_index + 7) % 12;
    return format!("{} - {} - {}", note, notes[third_index], notes[fifth_index]);
}