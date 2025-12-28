macro_rules! deps {
    () => {
        RiscvInterruptKind!();
    };
}

macro_rules! impl_285 {
    () => {
        deps!();
        impl RiscvInterruptKind { pub fn as_str (& self) -> & 'static str { match self { Self :: Machine => "machine" , Self :: Supervisor => "supervisor" , } } }
    };
}

impl_285!();