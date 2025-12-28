macro_rules! deps {
    () => {
        Sha256VarCore!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl fmt :: Debug for Sha256VarCore { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("Sha256VarCore { ... }") } }
    };
}

impl_9!()