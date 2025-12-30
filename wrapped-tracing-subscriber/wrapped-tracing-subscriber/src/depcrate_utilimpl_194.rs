// Generated macro for impl_194 (impl)
macro_rules! Depcrate_utilimpl_194 {
() => {
// Module: crate::util
// Provides: {"impl_194"}
// Dependencies: {}
impl fmt :: Debug for TryInitError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [cfg (feature = "std")] { fmt :: Debug :: fmt (& self . inner , f) } # [cfg (not (feature = "std"))] { f . write_str ("TryInitError(())") } } }
};
}
