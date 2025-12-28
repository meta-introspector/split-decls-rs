macro_rules! deps {
    () => {
        RefCount!();
        Config!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < C : cfg :: Config > Clone for RefCount < C > { fn clone (& self) -> Self { * self } }
    };
}

impl_110!();