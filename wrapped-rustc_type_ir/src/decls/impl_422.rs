macro_rules! deps {
    () => {
        WithCachedTypeInfo!();
    };
}

macro_rules! impl_422 {
    () => {
        deps!();
        impl < T > Deref for WithCachedTypeInfo < T > { type Target = T ; # [inline] fn deref (& self) -> & T { & self . internee } }
    };
}

impl_422!();