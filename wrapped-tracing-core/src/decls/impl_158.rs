macro_rules! deps {
    () => {
        DebugValue!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for DebugValue < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_158!();