// Generated macro for impl_30 (impl)
macro_rules! Depcrate_errorimpl_30 {
() => {
// Module: crate::error
// Provides: {"impl_30"}
// Dependencies: {}
impl Display for ErrorImpl < Erased > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("span backtrace:\n") ? ; Display :: fmt (& self . span_trace , f) } }
};
}
