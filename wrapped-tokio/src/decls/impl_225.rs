macro_rules! deps {
    () => {
        RwLockWriteGuard!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > Deref for RwLockWriteGuard < 'a , T > { type Target = T ; fn deref (& self) -> & T { self . 1 . deref () } }
    };
}

impl_225!()