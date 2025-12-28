macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! GROUP_MASK {
    () => {
        deps!();
        const GROUP_MASK : Opcode = (1 << GROUP_BITS) - 1 ;
    };
}

GROUP_MASK!();