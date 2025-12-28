macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! WRITE {
    () => {
        deps!();
        pub const WRITE : Opcode = 0x8000_0000 ;
    };
}

WRITE!()