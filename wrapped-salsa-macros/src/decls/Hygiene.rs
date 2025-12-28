macro_rules! Hygiene {
    () => {
        pub struct Hygiene { user_tokens : HashSet < String > , }
    };
}

Hygiene!();