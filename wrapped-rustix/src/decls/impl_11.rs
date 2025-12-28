macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < T , const N : usize > Buffer < T > for & mut [T ; N] { }
    };
}

impl_11!();