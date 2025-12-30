// Generated macro for Once (struct)
macro_rules! Depcrate_spin_onceOnce {
() => {
// Module: crate::spin::once
// Provides: {"Once"}
// Dependencies: {}
# [doc = " A synchronization primitive which can be used to run a one-time global"] # [doc = " initialization. Unlike its std equivalent, this is generalized so that the"] # [doc = " closure returns a value and it is stored. Once therefore acts something like"] # [doc = " a future, too."] pub struct Once < T > { state : AtomicUsize , data : UnsafeCell < Option < T > > , }
};
}
