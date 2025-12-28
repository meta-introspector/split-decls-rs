macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! _Opcode {
    () => {
        deps!();
        # [cfg (windows)] type _Opcode = i32 ;
    };
}

_Opcode!();