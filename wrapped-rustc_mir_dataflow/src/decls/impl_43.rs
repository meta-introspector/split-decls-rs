macro_rules! deps {
    () => {
        DebugWithContext!();
        Formatter!();
        DebugWithAdapter!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T , C > fmt :: Debug for DebugWithAdapter < '_ , T , C > where T : DebugWithContext < C > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . this . fmt_with (self . ctxt , f) } }
    };
}

impl_43!();