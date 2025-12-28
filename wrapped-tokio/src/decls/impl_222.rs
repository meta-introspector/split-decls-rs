macro_rules! deps {
    () => {
        MutexGuard!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > DerefMut for MutexGuard < 'a , T > { fn deref_mut (& mut self) -> & mut T { self . 1 . deref_mut () } }
    };
}

impl_222!();