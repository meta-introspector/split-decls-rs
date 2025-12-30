// Generated macro for impl_29 (impl)
macro_rules! Depcrate_errorimpl_29 {
() => {
// Module: crate::error
// Provides: {"impl_29"}
// Dependencies: {}
impl Debug for ErrorImpl < Erased > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("span backtrace:\n") ? ; Debug :: fmt (& self . span_trace , f) } }
};
}
