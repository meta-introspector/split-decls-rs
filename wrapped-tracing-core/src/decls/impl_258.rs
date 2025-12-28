macro_rules! deps {
    () => {
        Mutex!();
        MutexGuard!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl < T > Mutex < T > { pub (crate) fn lock (& self) -> Result < MutexGuard < '_ , T > , () > { Ok (self . inner . lock ()) } }
    };
}

impl_258!();