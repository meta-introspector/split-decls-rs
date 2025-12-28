macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! GROUP_SHIFT {
    () => {
        deps!();
        const GROUP_SHIFT : Opcode = NUM_SHIFT + NUM_BITS ;
    };
}

GROUP_SHIFT!()