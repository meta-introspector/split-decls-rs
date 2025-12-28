macro_rules! deps {
    () => {
        Variance!();
    };
}

macro_rules! impl_516 {
    () => {
        deps!();
        impl fmt :: Debug for Variance { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match * self { Variance :: Covariant => "+" , Variance :: Contravariant => "-" , Variance :: Invariant => "o" , Variance :: Bivariant => "*" , }) } }
    };
}

impl_516!()