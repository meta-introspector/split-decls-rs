macro_rules! deps {
    () => {
        TryFromSliceError!();
        ArrayVec!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl core :: fmt :: Display for TryFromSliceError { # [inline] fn fmt (& self , f : & mut Formatter < '_ >) -> core :: fmt :: Result { f . write_str ("could not convert slice to ArrayVec") } }
    };
}

impl_37!()