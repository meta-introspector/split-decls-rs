macro_rules! deps {
    () => {
        Interned!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl < 'a , T > Clone for Interned < 'a , T > { fn clone (& self) -> Self { * self } }
    };
}

impl_237!();