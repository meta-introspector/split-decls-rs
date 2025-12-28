macro_rules! deps {
    () => {
        MutexGuard!();
        Condvar!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl Condvar { # [inline] pub (crate) fn new () -> Condvar { Condvar (PhantomData , parking_lot :: Condvar :: new ()) } # [inline] pub (crate) fn notify_one (& self) { self . 1 . notify_one () ; } # [inline] pub (crate) fn notify_all (& self) { self . 1 . notify_all () ; } # [inline] pub (crate) fn wait < 'a , T > (& self , mut guard : MutexGuard < 'a , T > ,) -> LockResult < MutexGuard < 'a , T > > { self . 1 . wait (& mut guard . 1) ; Ok (guard) } # [inline] pub (crate) fn wait_timeout < 'a , T > (& self , mut guard : MutexGuard < 'a , T > , timeout : Duration ,) -> LockResult < (MutexGuard < 'a , T > , WaitTimeoutResult) > { let wtr = self . 1 . wait_for (& mut guard . 1 , timeout) ; Ok ((guard , wtr)) } }
    };
}

impl_227!()