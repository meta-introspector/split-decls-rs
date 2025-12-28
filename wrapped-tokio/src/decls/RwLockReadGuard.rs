macro_rules! RwLockReadGuard {
    () => {
        # [derive (Debug)] pub (crate) struct RwLockReadGuard < 'a , T : ? Sized > (PhantomData < std :: sync :: RwLockReadGuard < 'a , T > > , parking_lot :: RwLockReadGuard < 'a , T > ,) ;
    };
}

RwLockReadGuard!()