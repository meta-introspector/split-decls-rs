macro_rules! deps {
    () => {
        MutexGuard!();
        Mutex!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < T : ? Sized > Mutex < T > { fn obtain_lock (& self) { while self . lock . compare_exchange_weak (false , true , Ordering :: Acquire , Ordering :: Relaxed) . is_err () { while self . lock . load (Ordering :: Relaxed) { hint :: spin_loop () ; } } } # [doc = " Locks the spinlock and returns a guard."] # [doc = ""] # [doc = " The returned value may be dereferenced for data access"] # [doc = " and the lock will be dropped when the guard falls out of scope."] pub (crate) fn lock (& self) -> MutexGuard < '_ , T > { self . obtain_lock () ; MutexGuard { lock : & self . lock , data : unsafe { & mut * self . data . get () } , } } # [doc = " Tries to lock the mutex. If it is already locked, it will return None. Otherwise it returns"] # [doc = " a guard within Some."] pub (crate) fn try_lock (& self) -> Option < MutexGuard < '_ , T > > { if self . lock . compare_exchange (false , true , Ordering :: Acquire , Ordering :: Relaxed) . is_ok () { Some (MutexGuard { lock : & self . lock , data : unsafe { & mut * self . data . get () } , }) } else { None } } }
    };
}

impl_12!()