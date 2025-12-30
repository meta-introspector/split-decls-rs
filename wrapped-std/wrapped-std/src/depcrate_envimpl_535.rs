// Generated macro for impl_535 (impl)
macro_rules! Depcrate_envimpl_535 {
() => {
// Module: crate::env
// Provides: {"impl_535"}
// Dependencies: {}
# [stable (feature = "std_debug" , since = "1.16.0")] impl fmt :: Debug for Vars { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let Self { inner : VarsOs { inner } } = self ; f . debug_struct ("Vars") . field ("inner" , & inner . str_debug ()) . finish () } }
};
}
