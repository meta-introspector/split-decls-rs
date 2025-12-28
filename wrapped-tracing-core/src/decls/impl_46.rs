macro_rules! deps {
    () => {
        Identifier!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl fmt :: Debug for Identifier { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Identifier({:p})" , self . 0) } }
    };
}

impl_46!();