macro_rules! deps {
    () => {
        Registry!();
        CoreLatch!();
        SpinLatch!();
        Latch!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < 'r > Latch for SpinLatch < 'r > { # [inline] unsafe fn set (this : * const Self) { let cross_registry ; let registry : & Registry = if unsafe { (* this) . cross } { cross_registry = Arc :: clone (unsafe { (* this) . registry }) ; & cross_registry } else { unsafe { (* this) . registry } } ; let target_worker_index = unsafe { (* this) . target_worker_index } ; if unsafe { CoreLatch :: set (& (* this) . core_latch) } { registry . notify_worker_latch_is_set (target_worker_index) ; } } }
    };
}

impl_80!()