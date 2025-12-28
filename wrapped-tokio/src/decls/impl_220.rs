macro_rules! deps {
    () => {
        MutexGuard!();
        Mutex!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl < T > Mutex < T > { # [inline] pub (crate) fn new (t : T) -> Mutex < T > { Mutex (PhantomData , parking_lot :: Mutex :: new (t)) } # [inline] # [cfg (not (all (loom , test)))] pub (crate) const fn const_new (t : T) -> Mutex < T > { Mutex (PhantomData , parking_lot :: const_mutex (t)) } # [inline] pub (crate) fn lock (& self) -> MutexGuard < '_ , T > { MutexGuard (PhantomData , self . 1 . lock ()) } # [inline] pub (crate) fn try_lock (& self) -> Option < MutexGuard < '_ , T > > { self . 1 . try_lock () . map (| guard | MutexGuard (PhantomData , guard)) } # [inline] pub (crate) fn get_mut (& mut self) -> & mut T { self . 1 . get_mut () } }
    };
}

impl_220!()