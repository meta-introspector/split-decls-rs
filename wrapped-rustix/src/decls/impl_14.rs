macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < T , const N : usize > Buffer < T > for & mut [MaybeUninit < T > ; N] { }
    };
}

impl_14!()