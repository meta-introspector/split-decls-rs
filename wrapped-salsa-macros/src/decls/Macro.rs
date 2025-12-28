macro_rules! deps {
    () => {
        TrackedArgs!();
        Hygiene!();
    };
}

macro_rules! Macro {
    () => {
        deps!();
        struct Macro { hygiene : Hygiene , args : TrackedArgs , struct_item : syn :: ItemStruct , }
    };
}

Macro!();