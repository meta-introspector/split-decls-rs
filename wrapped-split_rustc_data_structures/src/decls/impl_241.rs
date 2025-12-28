macro_rules! deps {
    () => {
        Interned!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl < 'a , T > Eq for Interned < 'a , T > { }
    };
}

impl_241!();