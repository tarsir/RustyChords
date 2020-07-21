#[path = "chord_types.rs"]
pub mod chord_types;

pub mod config {
    use crate::chord_types::ChordTypes; // Q: why didn't this work on line 3?

    pub struct SessionConfig {
        pub chord_type: ChordTypes,
        pub add_seven: bool,
        pub show_flats: bool
    }

    pub fn default_config() -> SessionConfig {
        return SessionConfig {
            chord_type: ChordTypes::Major,
            add_seven: false,
            show_flats: false
        };
    }

    pub fn print_config(cfg: &SessionConfig) {
        println!("Your current configuration is: ");
        let c_type = match cfg.chord_type {
            ChordTypes::Major => "Major",
            ChordTypes::Minor => "Minor",
            ChordTypes::Augmented => "Aug",
            ChordTypes::Diminished => "Dim",
        };
        print!("{}", c_type);
        println!("{}", if cfg.add_seven { "7" } else { "" });
        println!("Showing as {}", if cfg.show_flats { "flats" } else { "sharps" });
    }
}