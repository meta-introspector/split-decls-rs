macro_rules! deps {
    () => {
        MutexGuard!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > Drop for MutexGuard < 'a , T > { # [doc = " The dropping of the MutexGuard will release the lock it was created from."] fn drop (& mut self) { self . lock . store (false , Ordering :: Release) ; } }
    };
}

impl_17!();