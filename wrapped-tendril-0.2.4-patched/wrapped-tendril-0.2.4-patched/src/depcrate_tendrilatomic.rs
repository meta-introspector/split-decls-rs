// Generated macro for Atomic (struct)
macro_rules! Depcrate_tendrilAtomic {
() => {
// Module: crate::tendril
// Provides: {"Atomic"}
// Dependencies: {}
# [doc = " A marker of an atomic (and hence concurrent) tendril."] # [doc = ""] # [doc = " This is used as the second, optional type parameter of a `Tendril`; `Tendril<F, Atomic>` thus"] # [doc = " implements both `Send` and `Sync`."] # [doc = ""] # [doc = " This is akin to using `Arc` for reference counting."] pub struct Atomic (AtomicUsize) ;
};
}
