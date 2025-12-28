macro_rules! deps {
    () => {
        ClosureKind!();
    };
}

macro_rules! impl_523 {
    () => {
        deps!();
        impl fmt :: Display for ClosureKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . as_str () . fmt (f) } }
    };
}

impl_523!();