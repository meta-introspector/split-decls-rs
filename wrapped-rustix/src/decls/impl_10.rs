macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T > Buffer < T > for & mut [T] { }
    };
}

impl_10!()