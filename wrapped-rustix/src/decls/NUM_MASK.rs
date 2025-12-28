macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! NUM_MASK {
    () => {
        deps!();
        const NUM_MASK : Opcode = (1 << NUM_BITS) - 1 ;
    };
}

NUM_MASK!();