macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < T > Buffer < T > for & mut [MaybeUninit < T >] { }
    };
}

impl_13!();