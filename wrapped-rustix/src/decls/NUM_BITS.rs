macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! NUM_BITS {
    () => {
        deps!();
        const NUM_BITS : Opcode = 8 ;
    };
}

NUM_BITS!();