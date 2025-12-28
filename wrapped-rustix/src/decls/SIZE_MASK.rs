macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! SIZE_MASK {
    () => {
        deps!();
        const SIZE_MASK : Opcode = (1 << SIZE_BITS) - 1 ;
    };
}

SIZE_MASK!();