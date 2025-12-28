macro_rules! deps {
    () => {
        Result!();
        NoArg!();
        Opcode!();
    };
}

macro_rules! impl_451 {
    () => {
        deps!();
        impl < const OPCODE : Opcode > fmt :: Debug for NoArg < OPCODE > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("NoArg") . field (& OPCODE) . finish () } }
    };
}

impl_451!()