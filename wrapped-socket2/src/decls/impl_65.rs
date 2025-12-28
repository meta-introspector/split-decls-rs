macro_rules! deps {
    () => {
        MaybeUninitSlice!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < 'a > fmt :: Debug for MaybeUninitSlice < 'a > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . 0 . as_slice () , fmt) } }
    };
}

impl_65!();