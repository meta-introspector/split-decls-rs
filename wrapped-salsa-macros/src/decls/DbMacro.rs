macro_rules! deps {
    () => {
        Hygiene!();
    };
}

macro_rules! DbMacro {
    () => {
        deps!();
        struct DbMacro { hygiene : Hygiene , }
    };
}

DbMacro!()