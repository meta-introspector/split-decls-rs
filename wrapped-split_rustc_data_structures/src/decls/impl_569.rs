macro_rules! deps {
    () => {
        MTLock!();
    };
}

macro_rules! impl_569 {
    () => {
        deps!();
        impl < T > MTLock < T > { # [inline (always)] pub fn new (inner : T) -> Self { MTLock (Lock :: new (inner)) } # [inline (always)] pub fn into_inner (self) -> T { self . 0 . into_inner () } # [inline (always)] pub fn get_mut (& mut self) -> & mut T { self . 0 . get_mut () } # [inline (always)] pub fn lock (& self) -> LockGuard < '_ , T > { self . 0 . lock () } # [inline (always)] pub fn lock_mut (& self) -> LockGuard < '_ , T > { self . lock () } }
    };
}

impl_569!()