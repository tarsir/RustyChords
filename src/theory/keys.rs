#[path = "notes.rs"]
mod notes;

pub fn get_key_signature(root_note: String, ref_notes: [&str; 12]) -> () {
    let intervals: [usize; 7] = [
       0, 2, 2, 1, 2, 2, 2
    ];

    let root_index: usize = match notes::get_note_index(&root_note, ref_notes) {
        Some(i) => i,
        None => return
    };

    let mut counter: usize = 0;
    let mut result: Vec<String> = Vec::new();

    for n in intervals.iter() {
        counter += *n;
        result.push(ref_notes[(root_index + counter) % 12].to_string());
    }

    result = if check_repeating_naturals(&result) {
        result.iter().map(|note| notes::note_as_flat(note, ref_notes)).collect()
    } else {
        result
    };

    println!("{}", result.join(","));
}

fn check_repeating_naturals(note_list: &Vec<String>) -> bool {
    let mut notes_iter = note_list.iter();
    let mut ref_character: char = notes_iter.next().unwrap().chars().nth(0).unwrap();
    let mut first: char;

    for ch in notes_iter {
        first = ch.chars().nth(0).unwrap();
        if first == ref_character {
            return true
        }

        ref_character = first;
    }

    false
}