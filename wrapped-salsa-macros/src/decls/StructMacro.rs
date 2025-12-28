macro_rules! deps {
    () => {
        Options!();
        Hygiene!();
        Accumulator!();
    };
}

macro_rules! StructMacro {
    () => {
        deps!();
        struct StructMacro { hygiene : Hygiene , _args : Options < Accumulator > , struct_item : syn :: ItemStruct , }
    };
}

StructMacro!()