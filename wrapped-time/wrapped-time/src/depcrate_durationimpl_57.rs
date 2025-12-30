// Generated macro for impl_57 (impl)
macro_rules! Depcrate_durationimpl_57 {
() => {
// Module: crate::duration
// Provides: {"impl_57"}
// Dependencies: {}
impl fmt :: Debug for Duration { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Duration") . field ("seconds" , & self . seconds) . field ("nanoseconds" , & self . nanoseconds) . finish () } }
};
}
