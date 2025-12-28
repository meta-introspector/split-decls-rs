macro_rules! deps {
    () => {
        Config!();
        Tid!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < C : cfg :: Config > Clone for Tid < C > { fn clone (& self) -> Self { * self } }
    };
}

impl_172!()