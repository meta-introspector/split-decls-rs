macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! DIR_SHIFT {
    () => {
        deps!();
        const DIR_SHIFT : Opcode = SIZE_SHIFT + SIZE_BITS ;
    };
}

DIR_SHIFT!()