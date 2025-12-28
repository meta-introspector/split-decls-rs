macro_rules! deps {
    () => {
        Interned!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        impl < 'a , T > Copy for Interned < 'a , T > { }
    };
}

impl_238!()