// Generated macro for impl_195 (impl)
macro_rules! Depcrate_utilimpl_195 {
() => {
// Module: crate::util
// Provides: {"impl_195"}
// Dependencies: {}
impl fmt :: Display for TryInitError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [cfg (feature = "std")] { fmt :: Display :: fmt (& self . inner , f) } # [cfg (not (feature = "std"))] { f . write_str ("failed to set global default subscriber") } } }
};
}
