macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl fmt :: Debug for SmolStr { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . as_str () , f) } }
    };
}

impl_19!()