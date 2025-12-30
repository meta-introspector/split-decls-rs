// Generated macro for Mutex (struct)
macro_rules! Depcrate_spin_mutexMutex {
() => {
// Module: crate::spin::mutex
// Provides: {"Mutex"}
// Dependencies: {}
# [doc = " This type provides MUTual EXclusion based on spinning."] pub (crate) struct Mutex < T : ? Sized > { lock : AtomicBool , data : UnsafeCell < T > , }
};
}
