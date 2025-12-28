macro_rules! deps {
    () => {
        Direction!();
        Opcode!();
    };
}

macro_rules! compose_opcode {
    () => {
        deps!();
        pub (super) const fn compose_opcode (dir : Direction , group : Opcode , num : Opcode , size : Opcode ,) -> Opcode { let dir = match dir { Direction :: None => NONE , Direction :: Read => READ , Direction :: Write => WRITE , Direction :: ReadWrite => READ | WRITE , } ; dir | num | (group << 8) | ((size & IOCPARAM_MASK) << 16) }
    };
}

compose_opcode!();