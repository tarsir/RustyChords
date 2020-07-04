pub mod config {
    pub struct SessionConfig {
        pub is_major: bool,
        pub add_seven: bool
    }

    pub fn default_config() -> SessionConfig {
        return SessionConfig {
            is_major: true,
            add_seven: false
        };
    }

    pub fn print_config(cfg: &SessionConfig) {
        println!("Your current configuration is: ");
        print!("{}", if cfg.is_major { "Major" } else { "Minor" });
        println!("{}", if cfg.add_seven { "7" } else { "" });
    }
}