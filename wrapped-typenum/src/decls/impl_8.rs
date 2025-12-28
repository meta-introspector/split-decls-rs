macro_rules! deps {
    () => {
        PowerOfTwo!();
        B1!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl PowerOfTwo for B1 { }
    };
}

impl_8!()