macro_rules! deps {
    () => {
        Generation!();
        Config!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < C : cfg :: Config > Clone for Generation < C > { fn clone (& self) -> Self { * self } }
    };
}

impl_95!();