macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! NONE {
    () => {
        deps!();
        pub const NONE : Opcode = 0x2000_0000 ;
    };
}

NONE!()