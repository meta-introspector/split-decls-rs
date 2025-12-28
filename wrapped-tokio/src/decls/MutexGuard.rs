macro_rules! MutexGuard {
    () => {
        # [derive (Debug)] pub (crate) struct MutexGuard < 'a , T : ? Sized > (PhantomData < std :: sync :: MutexGuard < 'a , T > > , parking_lot :: MutexGuard < 'a , T > ,) ;
    };
}

MutexGuard!()