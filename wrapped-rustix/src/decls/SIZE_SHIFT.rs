macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! SIZE_SHIFT {
    () => {
        deps!();
        const SIZE_SHIFT : Opcode = GROUP_SHIFT + GROUP_BITS ;
    };
}

SIZE_SHIFT!()