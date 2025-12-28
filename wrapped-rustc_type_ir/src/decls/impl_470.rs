macro_rules! deps {
    () => {
        Ty!();
        FnSigTys!();
        Interner!();
    };
}

macro_rules! impl_470 {
    () => {
        deps!();
        impl < I : Interner > FnSigTys < I > { pub fn inputs (self) -> I :: FnInputTys { self . inputs_and_output . inputs () } pub fn output (self) -> I :: Ty { self . inputs_and_output . output () } }
    };
}

impl_470!()