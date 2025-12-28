macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl Sealed for usize { }
    };
}

impl_75!()