macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! DIR_MASK {
    () => {
        deps!();
        const DIR_MASK : Opcode = (1 << DIR_BITS) - 1 ;
    };
}

DIR_MASK!()