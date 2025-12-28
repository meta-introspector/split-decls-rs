macro_rules! deps {
    () => {
        RwLockReadGuard!();
        RwLock!();
        RwLockWriteGuard!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl < T > RwLock < T > { pub (crate) fn new (t : T) -> RwLock < T > { RwLock (PhantomData , parking_lot :: RwLock :: new (t)) } pub (crate) fn read (& self) -> RwLockReadGuard < '_ , T > { RwLockReadGuard (PhantomData , self . 1 . read ()) } pub (crate) fn try_read (& self) -> Option < RwLockReadGuard < '_ , T > > { self . 1 . try_read () . map (| guard | RwLockReadGuard (PhantomData , guard)) } pub (crate) fn write (& self) -> RwLockWriteGuard < '_ , T > { RwLockWriteGuard (PhantomData , self . 1 . write ()) } pub (crate) fn try_write (& self) -> Option < RwLockWriteGuard < '_ , T > > { self . 1 . try_write () . map (| guard | RwLockWriteGuard (PhantomData , guard)) } }
    };
}

impl_223!()