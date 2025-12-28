macro_rules! Mutex {
    () => {
        # [derive (Debug)] pub (crate) struct Mutex < T : ? Sized > (PhantomData < std :: sync :: Mutex < T > > , parking_lot :: Mutex < T >) ;
    };
}

Mutex!()