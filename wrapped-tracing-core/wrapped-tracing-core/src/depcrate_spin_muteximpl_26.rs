// Generated macro for impl_26 (impl)
macro_rules! Depcrate_spin_muteximpl_26 {
() => {
// Module: crate::spin::mutex
// Provides: {"impl_26"}
// Dependencies: {}
impl < T > Mutex < T > { # [doc = " Creates a new spinlock wrapping the supplied data."] pub (crate) const fn new (user_data : T) -> Mutex < T > { Mutex { lock : AtomicBool :: new (false) , data : UnsafeCell :: new (user_data) , } } }
};
}
