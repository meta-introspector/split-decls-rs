macro_rules! deps {
    () => {
        Getter!();
        Opcode!();
        Result!();
    };
}

macro_rules! impl_455 {
    () => {
        deps!();
        impl < const OPCODE : Opcode , Output > fmt :: Debug for Getter < OPCODE , Output > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Getter") . field (& OPCODE) . finish () } }
    };
}

impl_455!();