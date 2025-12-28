macro_rules! RwLockWriteGuard {
    () => {
        # [derive (Debug)] pub (crate) struct RwLockWriteGuard < 'a , T : ? Sized > (PhantomData < std :: sync :: RwLockWriteGuard < 'a , T > > , parking_lot :: RwLockWriteGuard < 'a , T > ,) ;
    };
}

RwLockWriteGuard!();