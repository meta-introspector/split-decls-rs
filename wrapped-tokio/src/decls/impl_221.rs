macro_rules! deps {
    () => {
        MutexGuard!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > Deref for MutexGuard < 'a , T > { type Target = T ; fn deref (& self) -> & T { self . 1 . deref () } }
    };
}

impl_221!()