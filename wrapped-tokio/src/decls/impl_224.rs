macro_rules! deps {
    () => {
        RwLockReadGuard!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > Deref for RwLockReadGuard < 'a , T > { type Target = T ; fn deref (& self) -> & T { self . 1 . deref () } }
    };
}

impl_224!();