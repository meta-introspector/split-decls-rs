macro_rules! Condvar {
    () => {
        # [derive (Debug)] pub (crate) struct Condvar (PhantomData < std :: sync :: Condvar > , parking_lot :: Condvar) ;
    };
}

Condvar!()