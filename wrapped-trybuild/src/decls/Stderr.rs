macro_rules! deps {
    () => {
        Variations!();
    };
}

macro_rules! Stderr {
    () => {
        deps!();
        struct Stderr { success : bool , stderr : Variations , }
    };
}

Stderr!();