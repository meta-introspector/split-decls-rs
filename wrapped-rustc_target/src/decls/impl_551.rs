macro_rules! deps {
    () => {
        TargetTuple!();
    };
}

macro_rules! impl_551 {
    () => {
        deps!();
        impl fmt :: Display for TargetTuple { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . debug_tuple ()) } }
    };
}

impl_551!();