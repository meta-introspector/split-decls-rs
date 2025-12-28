macro_rules! deps {
    () => {
        ThreadInfo!();
        LockLatch!();
        JobRef!();
        OnceLatch!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl ThreadInfo { fn new (stealer : Stealer < JobRef >) -> ThreadInfo { ThreadInfo { primed : LockLatch :: new () , stopped : LockLatch :: new () , terminate : OnceLatch :: new () , stealer , } } }
    };
}

impl_121!()