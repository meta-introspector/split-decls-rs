macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! GROUP_BITS {
    () => {
        deps!();
        const GROUP_BITS : Opcode = 8 ;
    };
}

GROUP_BITS!()