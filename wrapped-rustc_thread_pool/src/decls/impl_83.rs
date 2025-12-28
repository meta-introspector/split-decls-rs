macro_rules! deps {
    () => {
        Latch!();
        LockLatch!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl Latch for LockLatch { # [inline] unsafe fn set (this : * const Self) { let mut guard = unsafe { (* this) . m . lock () . unwrap () } ; * guard = true ; unsafe { (* this) . v . notify_all () } ; } }
    };
}

impl_83!();