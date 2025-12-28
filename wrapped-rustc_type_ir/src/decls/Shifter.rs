macro_rules! deps {
    () => {
        Interner!();
    };
}

macro_rules! Shifter {
    () => {
        deps!();
        struct Shifter < I : Interner > { cx : I , current_index : ty :: DebruijnIndex , amount : u32 , }
    };
}

Shifter!();