macro_rules! deps {
    () => {
        Sha512VarCore!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl fmt :: Debug for Sha512VarCore { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("Sha512VarCore { ... }") } }
    };
}

impl_21!()