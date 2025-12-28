macro_rules! deps {
    () => {
        MoveOut!();
        Formatter!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl fmt :: Debug for MoveOut { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "{:?}@{:?}" , self . path , self . source) } }
    };
}

impl_198!()