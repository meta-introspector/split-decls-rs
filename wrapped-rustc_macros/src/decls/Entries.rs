macro_rules! deps {
    () => {
        Predefined!();
    };
}

macro_rules! Entries {
    () => {
        deps!();
        struct Entries { map : HashMap < String , Predefined > , }
    };
}

Entries!();