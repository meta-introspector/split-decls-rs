macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! NUM_SHIFT {
    () => {
        deps!();
        const NUM_SHIFT : Opcode = 0 ;
    };
}

NUM_SHIFT!();