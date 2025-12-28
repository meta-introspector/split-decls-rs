macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl Sealed for usize { }
    };
}

impl_226!()