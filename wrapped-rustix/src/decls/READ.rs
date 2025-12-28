macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! READ {
    () => {
        deps!();
        pub const READ : Opcode = 0x4000_0000 ;
    };
}

READ!()