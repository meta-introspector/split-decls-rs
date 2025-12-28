macro_rules! impl_90 {
    () => {
        impl < T : ? Sized > Deref for Ref < '_ , T > { type Target = T ; fn deref (& self) -> & Self :: Target { self . 0 } }
    };
}

impl_90!()