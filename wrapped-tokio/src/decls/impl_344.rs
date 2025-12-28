macro_rules! deps {
    () => {
        CachePadded!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl < T > Deref for CachePadded < T > { type Target = T ; fn deref (& self) -> & T { & self . value } }
    };
}

impl_344!()