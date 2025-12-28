macro_rules! deps {
    () => {
        TempPath!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl fmt :: Debug for TempPath { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . path . fmt (f) } }
    };
}

impl_35!()