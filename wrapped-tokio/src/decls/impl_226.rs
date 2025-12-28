macro_rules! deps {
    () => {
        RwLockWriteGuard!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > DerefMut for RwLockWriteGuard < 'a , T > { fn deref_mut (& mut self) -> & mut T { self . 1 . deref_mut () } }
    };
}

impl_226!();