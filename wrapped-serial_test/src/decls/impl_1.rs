macro_rules! deps {
    () => {
        MutexGuardWrapper!();
        UniqueReentrantMutex!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl UniqueReentrantMutex { pub (crate) fn lock (& self) -> MutexGuardWrapper < '_ > { self . locks . serial () } pub (crate) fn start_parallel (& self) { self . locks . start_parallel () ; } pub (crate) fn end_parallel (& self) { self . locks . end_parallel () ; } # [cfg (test)] pub fn parallel_count (& self) -> u32 { self . locks . parallel_count () } # [cfg (test)] pub fn is_locked (& self) -> bool { self . locks . is_locked () } pub fn is_locked_by_current_thread (& self) -> bool { self . locks . is_locked_by_current_thread () } }
    };
}

impl_1!();