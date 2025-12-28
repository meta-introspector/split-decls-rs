macro_rules! deps {
    () => {
        Setter!();
        Result!();
        Opcode!();
    };
}

macro_rules! impl_459 {
    () => {
        deps!();
        impl < const OPCODE : Opcode , Input : fmt :: Debug > fmt :: Debug for Setter < OPCODE , Input > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Setter") . field (& OPCODE) . field (& self . input) . finish () } }
    };
}

impl_459!();