macro_rules! MutexGuard {
    () => {
        # [doc = " A guard to which the protected data can be accessed"] # [doc = ""] # [doc = " When the guard falls out of scope it will release the lock."] # [derive (Debug)] pub (crate) struct MutexGuard < 'a , T : ? Sized > { lock : & 'a AtomicBool , data : & 'a mut T , }
    };
}

MutexGuard!()