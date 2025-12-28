macro_rules! impl_91 {
    () => {
        impl < T : ? Sized > Deref for RefMut < '_ , T > { type Target = T ; fn deref (& self) -> & Self :: Target { self . 0 } }
    };
}

impl_91!()