macro_rules! deps {
    () => {
        Tid!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < C > Eq for Tid < C > { }
    };
}

impl_171!()