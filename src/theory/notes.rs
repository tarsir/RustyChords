pub fn get_note_index(note: &String, n_notes: [&str; 12]) -> Option<usize> {
    let mut index_ptr: usize = 0;
    let mut found: bool = false;

    for n in n_notes.iter() {
        match *n == note.trim() {
            true => found = true,
            _ => (index_ptr += 1)
        }

        if found {
            break;
        }
    }
    if found {
        return Some(index_ptr)
    } else {
        return None
    }
}

// TODO: is this function returning String sub-optimal?
pub fn note_as_flat(note: &String, notes: [&str; 12]) -> String {
    let note_index: usize;
    match get_note_index(note, notes) {
        Some(i) => note_index = i,
        None => {
            return note.to_string();
        }
    };

    let next_step = notes[(note_index + 1) % 12];
    if note.chars().count() == 2 || (note.chars().count() == 1 && next_step.chars().count() == 1) {
        return format!("{}b", next_step);
    }
    return note.to_string();
}