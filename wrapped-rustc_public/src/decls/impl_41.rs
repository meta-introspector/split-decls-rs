macro_rules! deps {
    () => {
        WrappingRange!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl Debug for WrappingRange { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . start > self . end { write ! (fmt , "(..={}) | ({}..)" , self . end , self . start) ? ; } else { write ! (fmt , "{}..={}" , self . start , self . end) ? ; } Ok (()) } }
    };
}

impl_41!();