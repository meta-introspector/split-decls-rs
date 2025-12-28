macro_rules! deps {
    () => {
        Formatter!();
        Init!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl fmt :: Debug for Init { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "{:?}@{:?} ({:?})" , self . path , self . location , self . kind) } }
    };
}

impl_202!()