macro_rules! deps {
    () => {
        Config!();
        Addr!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < C : cfg :: Config > Clone for Addr < C > { fn clone (& self) -> Self { * self } }
    };
}

impl_142!()