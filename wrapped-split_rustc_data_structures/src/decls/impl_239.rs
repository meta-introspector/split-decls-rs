macro_rules! deps {
    () => {
        Interned!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl < 'a , T > Deref for Interned < 'a , T > { type Target = T ; # [inline] fn deref (& self) -> & T { self . 0 } }
    };
}

impl_239!()