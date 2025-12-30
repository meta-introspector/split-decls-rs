// Generated macro for impl_2428 (impl)
macro_rules! Depcrate_kernel_sigsetimpl_2428 {
() => {
// Module: crate::kernel_sigset
// Provides: {"impl_2428"}
// Dependencies: {}
impl fmt :: Debug for KernelSigSet { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_set () ; for i in 1 ..= _NSIG { let sig = unsafe { Signal :: from_raw_unchecked (i as _) } ; if self . contains (sig) { d . entry (& sig) ; } } d . finish () } }
};
}
