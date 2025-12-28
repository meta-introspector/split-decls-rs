macro_rules! deps {
    () => {
        SetGlobalDefaultError!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl fmt :: Display for SetGlobalDefaultError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (Self :: MESSAGE) } }
    };
}

impl_77!()